use crate::models::session::{Session, SessionId};
use crate::models::user::User;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use tracing::*;
use warp::{http::HeaderValue, hyper::HeaderMap, reply::json, Filter};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    token: SessionId,
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
        todo!()
    }

    let session = sqlx::query_as!(
        Session,
        "INSERT INTO sessions (owner_id) VALUES ($1) RETURNING *",
        user.id
    )
    .fetch_one(&db)
    .await
    .unwrap();

    let body = LoginResponse { token: session.id };

    let response = warp::http::Response::builder()
        .header("Authorization", body.token.to_string())
        .header(
            "Set-Cookie",
            format!("authorization={}; Secure; HttpOnly", body.token),
        )
        .header("Content-Type", "application/json")
        .status(200)
        .body(serde_json::to_string(&body).unwrap())
        .unwrap();

    Ok(response)
}

pub async fn logout(
    db: crate::Db,
    header_session_id: Option<SessionId>,
    cookie_session_id: Option<SessionId>,
) -> Result<impl warp::Reply, Infallible> {
    let session = sqlx::query_as!(
        Session,
        "DELETE FROM sessions where id = $1 RETURNING *",
        cookie_session_id.unwrap()
    )
    .fetch_one(&db)
    .await
    .unwrap();
    debug!(?session, "session destroyed");

    Ok(warp::http::StatusCode::OK)
}
