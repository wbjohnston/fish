use std::convert::Infallible;

use warp::Reply;

use crate::models::user::{SanitizedUser, User};

pub async fn list() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("<h1>hello</h1>"))
}

pub async fn create(
    db: crate::Db,
    new_user: crate::models::user::NewUser,
) -> Result<impl Reply, Infallible> {
    let config = argon2::Config::default();
    // TODO(will): randomly generate salt
    let salt = b"randomsalt";

    // TODO(will): when can this fail?
    let hash = argon2::hash_encoded(new_user.password.as_bytes(), salt, &config).unwrap();

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

pub async fn update() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("<h1>hello</h1>"))
}

pub async fn fetch(id: String) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html(format!("<h1>{}</h1>", id)))
}
