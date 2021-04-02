use std::sync::Arc;

use tokio::sync::Mutex;
use warp::Filter;

use crate::types::State;

pub fn index(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(db.clone())
        .or(create(db.clone()))
        .or(update(db.clone()))
        .or(fetch(db.clone()))
        .or(ws(db))
}

fn list(db: crate::Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client")
        .and(warp::get())
        .and_then(crate::handlers::client::list)
}

fn create(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client")
        .and(warp::post())
        .and_then(crate::handlers::client::create)
}

fn update(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client")
        .and(warp::put())
        .and_then(crate::handlers::client::update)
}

fn fetch(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client" / String)
        .and(warp::get())
        .and_then(crate::handlers::client::fetch)
}

/// WS /bot/:id
fn ws(db: crate::Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client" / u32)
        .and(warp::ws())
        .map(|id, ws: warp::ws::Ws| {
            ws.on_upgrade(move |socket| crate::handlers::client::ws(id, socket))
        })
}
