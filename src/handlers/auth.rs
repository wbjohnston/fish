use crate::models::user::User;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    token: String,
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

    // TODO(will): create session
    let response = LoginResponse {
        token: "foobar".to_string(),
    };
    Ok(warp::reply::json(&response))
}
