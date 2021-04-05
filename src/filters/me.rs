use crate::models::session::Session;

use super::auth::authorization_token_filter;
use warp::Filter;

pub fn index(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    fetch(db.clone()).or(ws(db.clone()))
}

pub fn fetch(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("me")
        .and(warp::get())
        .and(authorization_token_filter(db.clone()))
        .and_then(move |session: Session| {
            crate::handlers::user::fetch(db.clone(), session.owner_id)
        })
}

pub fn ws(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("me" / "ws")
        .and(authorization_token_filter(db.clone()))
        .and(warp::ws())
        .and_then(move |session: Session, ws: warp::ws::Ws| {
            let id = session.owner_id.clone();
            crate::handlers::user::ws(db.clone(), session, id, ws)
        })
}
