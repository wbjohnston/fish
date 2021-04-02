use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;
use warp::Filter;

pub fn index(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(db.clone())
        .or(create(db.clone()))
        .or(fetch(db))
}

fn list(db: crate::Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::get())
        .and_then(move || crate::handlers::user::list(db.clone()))
}

fn create(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |user: crate::models::user::NewUser| {
            crate::handlers::user::create(db.clone(), user)
        })
}

fn fetch(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user" / Uuid)
        .and(warp::get())
        .and_then(move |id| crate::handlers::user::fetch(db.clone(), id))
}
