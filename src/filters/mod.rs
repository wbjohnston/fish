pub mod auth;
pub mod game;
pub mod me;
pub mod user;

use warp::Filter;

pub fn index(
    context: crate::Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let index = warp::path::end().map(|| warp::reply::html("not found"));

    index
        .or(auth::index(context.clone()))
        .or(game::index(context.clone()))
        .or(user::index(context.clone()))
        .or(me::index(context.clone()))
}
