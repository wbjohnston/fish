use warp::Filter;

use super::auth::authorization_token_filter;

pub fn index(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(db.clone()).or(create(db))
}

/// GET /game
fn list(db: crate::Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("game")
        .and(warp::get())
        // .and(authorization_token_filter(db.clone()))
        .and_then(move || crate::handlers::game::list(db.clone()))
}

/// POST /game
fn create(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("game")
        .and(warp::post())
        .and(authorization_token_filter(db.clone()))
        .and(warp::body::json())
        .and_then(move |session, new_game| {
            crate::handlers::game::create(db.clone(), session, new_game)
        })
}
