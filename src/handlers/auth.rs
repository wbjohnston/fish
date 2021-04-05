use crate::models::session::{Session, SessionId};
use crate::models::user::User;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::convert::Infallible;
use warp::Rejection;

#[derive(Debug)]
pub enum AuthError {
    NotAuthorized,
}

impl warp::reject::Reject for AuthError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn authorize(
    db: crate::Db,
    header: Option<SessionId>,
    cookie: Option<SessionId>,
) -> Result<Session, Rejection> {
    let session_id = match (header, cookie) {
        (Some(x), _) => x,
        (_, Some(x)) => x,
        _ => return Err(warp::reject::custom(AuthError::NotAuthorized)),
    };

    let session = match sqlx::query_as!(Session, "SELECT * FROM sessions where id = $1", session_id)
        .fetch_one(&db)
        .await
    {
        Ok(session) => session,
        Err(_) => return Err(warp::reject::custom(AuthError::NotAuthorized)),
    };

    Ok(session)
}

pub async fn login(db: crate::Db, req: LoginRequest) -> Result<impl warp::Reply, Infallible> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        req.username
    )
    .fetch_one(&db)
    .await
    .unwrap();

    if !crate::services::auth::verify_matches(req.password.as_bytes(), user.password_hash.as_str())
    {
        let response = warp::http::Response::builder()
            .status(warp::http::StatusCode::UNAUTHORIZED)
            .body(String::new())
            .unwrap();

        return Ok(response);
    }

    let session = sqlx::query_as!(
        Session,
        "INSERT INTO sessions (owner_id) VALUES ($1) RETURNING *",
        user.id
    )
    .fetch_one(&db)
    .await
    .unwrap();

    let response = warp::http::Response::builder()
        .header("Authorization", session.id.to_string())
        .header(
            "Set-Cookie",
            format!("authorization={}; path=/; HttpOnly;;", session.id),
        )
        .header("Content-Type", "application/json")
        .status(200)
        .body(json!({"token": session.id}).to_string())
        .unwrap();

    Ok(response)
}

pub async fn logout(db: crate::Db, session: Session) -> Result<impl warp::Reply, Infallible> {
    let _ = sqlx::query_as!(
        Session,
        "DELETE FROM sessions WHERE id = $1 RETURNING *",
        session.id,
    )
    .fetch_one(&db)
    .await
    .unwrap();

    let response = warp::http::Response::builder()
        .header(
            "Set-Cookie",
            "authorization=deleted; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT",
        )
        .status(200)
        .body(String::new())
        .unwrap();

    Ok(response)
}
