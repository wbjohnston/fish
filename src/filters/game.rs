use warp::Filter;

use crate::models::game::GameId;

use super::auth::authorization_token_filter;

pub fn index(
    context: crate::Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(context.clone())
        .or(fetch(context.clone()))
        .or(create(context))
}

/// GET /game
fn list(
    context: crate::Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("game")
        .and(warp::get())
        // .and(authorization_token_filter(db.clone()))
        .and_then(move || crate::handlers::game::list(context.db.clone()))
}
fn fetch(
    context: crate::Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("game" / GameId)
        .and(warp::get())
        .and_then(move |id| crate::handlers::game::fetch(context.db.clone(), id))
}

/// POST /game
fn create(
    context: crate::Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("game")
        .and(warp::post())
        .and(authorization_token_filter(context.clone()))
        .and(warp::body::json())
        .and_then(move |session, new_game| {
            crate::handlers::game::create(context.db.clone(), session, new_game)
        })
}
