use std::sync::Arc;

use tokio::sync::Mutex;
use warp::Filter;

use crate::types::State;

pub fn index(
    state: Arc<Mutex<State>>,
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(state.clone(), db.clone()).or(create(state, db))
}

/// GET /room
fn list(
    state: Arc<Mutex<State>>,
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("room")
        .and(warp::get())
        .and_then(move || crate::handlers::room::list(state.clone()))
}

/// POST /room
fn create(
    state: Arc<Mutex<State>>,
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("room")
        .and(warp::post())
        .and_then(move || crate::handlers::room::create(state.clone()))
}
