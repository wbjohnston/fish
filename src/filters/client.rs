use std::sync::Arc;

use tokio::sync::Mutex;
use warp::Filter;

use crate::types::State;

pub fn index(
    state: Arc<Mutex<State>>,
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(state.clone(), db.clone())
        .or(create(state.clone(), db.clone()))
        .or(update(state.clone(), db.clone()))
        .or(fetch(state.clone(), db.clone()))
        .or(ws(state, db))
}

fn list(
    state: Arc<Mutex<State>>,
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client")
        .and(warp::get())
        .and_then(crate::handlers::client::list)
}

fn create(
    state: Arc<Mutex<State>>,
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client")
        .and(warp::post())
        .and_then(crate::handlers::client::create)
}

fn update(
    state: Arc<Mutex<State>>,
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client")
        .and(warp::put())
        .and_then(crate::handlers::client::update)
}

fn fetch(
    state: Arc<Mutex<State>>,
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client" / String)
        .and(warp::get())
        .and_then(crate::handlers::client::fetch)
}

/// WS /bot/:id
fn ws(
    state: Arc<Mutex<State>>,
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client" / u32)
        .and(warp::ws())
        .map(|id, ws: warp::ws::Ws| {
            ws.on_upgrade(move |socket| crate::handlers::client::ws(id, socket))
        })
}
