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
        .or(fetch(state))
}

fn list(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::get())
        .and_then(crate::handlers::user::list)
}

fn create(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::post())
        .and_then(crate::handlers::user::create)
}

fn update(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::put())
        .and_then(crate::handlers::user::update)
}

fn fetch(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user" / String)
        .and(warp::get())
        .and_then(crate::handlers::user::fetch)
}
