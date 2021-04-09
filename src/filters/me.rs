use crate::models::session::Session;
use crate::prelude::*;

use super::auth::authorization_token_filter;
use warp::Filter;

pub fn index(
    context: Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    fetch(context.clone()).or(ws(context.clone()))
}

pub fn fetch(
    context: Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("me")
        .and(warp::get())
        .and(authorization_token_filter(context.clone()))
        .and_then(move |session: Session| {
            crate::handlers::user::fetch(context.db.clone(), session.owner_id)
        })
}

pub fn ws(
    context: Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("me" / "ws")
        .and(authorization_token_filter(context.clone()))
        .and(warp::ws())
        .and_then(move |session: Session, ws: warp::ws::Ws| {
            let id = session.owner_id.clone();
            crate::handlers::user::ws(context.clone(), session, id, ws)
        })
}
