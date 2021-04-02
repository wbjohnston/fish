use std::sync::Arc;

use tokio::sync::Mutex;
use warp::Filter;

use crate::types::State;

pub fn index(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(state.clone()).or(create(state))
}

/// GET /room
fn list(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("room")
        .and(warp::get())
        .and_then(move || crate::handlers::room::list(state.clone()))
}

/// POST /room
fn create(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("room")
        .and(warp::post())
        .and_then(move || crate::handlers::room::create(state.clone()))
}
