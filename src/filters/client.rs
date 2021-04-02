use std::sync::Arc;

use tokio::sync::Mutex;
use warp::Filter;

use crate::types::State;

pub fn index(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(state.clone())
        .or(create(state.clone()))
        .or(update(state.clone()))
        .or(fetch(state.clone()))
        .or(ws(state))
}

fn list(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client")
        .and(warp::get())
        .and_then(crate::handlers::client::list)
}

fn create(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client")
        .and(warp::post())
        .and_then(crate::handlers::client::create)
}

fn update(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client")
        .and(warp::put())
        .and_then(crate::handlers::client::update)
}

fn fetch(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client" / String)
        .and(warp::get())
        .and_then(crate::handlers::client::fetch)
}

/// WS /bot/:id
fn ws(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client" / u32)
        .and(warp::ws())
        .map(|id, ws: warp::ws::Ws| {
            ws.on_upgrade(move |socket| crate::handlers::client::ws(id, socket))
        })
}
