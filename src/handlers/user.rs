use std::convert::Infallible;

use uuid::Uuid;
use warp::Reply;

use crate::{
    models::user::{SanitizedUser, User},
    services::auth::hash_password,
};

pub async fn list(db: crate::Db) -> Result<impl warp::Reply, Infallible> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&db)
        .await
        .unwrap();

    let sanitized: Vec<_> = users.into_iter().map(SanitizedUser::from).collect();

    Ok(warp::reply::json(&sanitized))
}

pub async fn create(
    db: crate::Db,
    new_user: crate::models::user::NewUser,
) -> Result<impl Reply, Infallible> {
    let hash = hash_password(new_user.password.as_bytes());

    let user: User = sqlx::query_as!(
        User,
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING *",
        new_user.username,
        hash,
    )
    .fetch_one(&db)
    .await
    .unwrap();

    let sanitized = SanitizedUser::from(user);

    Ok(warp::reply::json(&sanitized))
}

pub async fn fetch(db: crate::Db, id: Uuid) -> Result<impl warp::Reply, Infallible> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1 LIMIT 1", id)
        .fetch_one(&db)
        .await
        .unwrap();

    let sanitized = SanitizedUser::from(user);

    Ok(warp::reply::json(&sanitized))
}
