mod client;
mod room;
mod user;
use std::sync::Arc;

use tokio::sync::Mutex;
use warp::Filter;

use crate::types::State;

pub fn index(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let index = warp::path::end().map(|| warp::reply::html("not found"));

    index
        .or(client::index(state.clone()))
        .or(room::index(state.clone()))
        .or(user::index(state))
}
