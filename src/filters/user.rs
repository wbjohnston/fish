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
        .or(fetch(state, db))
}

fn list(
    state: Arc<Mutex<State>>,
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::get())
        .and_then(crate::handlers::user::list)
}

fn create(
    state: Arc<Mutex<State>>,
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::post())
        .and_then(crate::handlers::user::create)
}

fn update(
    state: Arc<Mutex<State>>,
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::put())
        .and_then(crate::handlers::user::update)
}

fn fetch(
    state: Arc<Mutex<State>>,
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user" / String)
        .and(warp::get())
        .and_then(crate::handlers::user::fetch)
}
