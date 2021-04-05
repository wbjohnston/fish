use warp::Filter;

use crate::models::client::ClientId;

use super::auth::authorization_token_filter;

pub fn index(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(db.clone())
        .or(create(db.clone()))
        .or(fetch(db.clone()))
        .or(ws(db))
}

/// GET /client
fn list(db: crate::Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client")
        .and(warp::get())
        // .and(authorization_token_filter(db.clone()))
        .and_then(move || crate::handlers::client::list(db.clone()))
}

/// POST /client
fn create(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client")
        .and(warp::post())
        .and(authorization_token_filter(db.clone()))
        .and(warp::body::json())
        .and_then(move |session, new_client| {
            crate::handlers::client::create(db.clone(), session, new_client)
        })
}

/// GET /client/:id
fn fetch(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client" / ClientId)
        .and(warp::get())
        .and(authorization_token_filter(db.clone()))
        .and_then(move |id: ClientId, session| {
            crate::handlers::client::fetch(db.clone(), session, id)
        })
}

/// WS /clietn/:id/ws
fn ws(db: crate::Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client" / ClientId / "ws")
        .and(authorization_token_filter(db.clone()))
        .and(warp::ws())
        .and_then(move |id, session, ws: warp::ws::Ws| {
            crate::handlers::client::ws(db.clone(), session, id, ws)
        })
}
