use warp::Filter;

use crate::models::client::ClientId;

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
        .and_then(move || crate::handlers::client::list(db.clone()))
}

fn create(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client")
        .and(warp::post())
        .and_then(move || crate::handlers::client::create(db.clone()))
}

fn fetch(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client" / ClientId)
        .and(warp::get())
        .and_then(move |id: ClientId| crate::handlers::client::fetch(db.clone(), id))
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
