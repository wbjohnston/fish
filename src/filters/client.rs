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

fn list(db: crate::Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client")
        .and(warp::get())
        .and(authorization_token_filter(db.clone()))
        .and_then(move |session| crate::handlers::client::list(db.clone(), session))
}

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

/// WS /bot/:id
fn ws(db: crate::Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client" / ClientId)
        .and(warp::ws())
        .map(move |id, ws: warp::ws::Ws| {
            let db = db.clone();
            ws.on_upgrade(move |socket| crate::handlers::client::ws(db.clone(), id, socket))
        })
}
