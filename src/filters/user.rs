use uuid::Uuid;
use warp::Filter;

use crate::models::user::UserId;

use super::auth::authorization_token_filter;

pub fn index(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(db.clone()).or(create(db.clone())).or(fetch(db))
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

/// WS /user/:id/ws
fn ws(db: crate::Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user" / UserId / "ws")
        .and(authorization_token_filter(db.clone()))
        .and(warp::ws())
        .and_then(move |id, session, ws: warp::ws::Ws| {
            crate::handlers::user::ws(db.clone(), session, id, ws)
        })
}
