use warp::Filter;

pub fn index(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(db.clone()).or(create(db))
}

/// GET /room
fn list(db: crate::Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("room")
        .and(warp::get())
        .and_then(move || crate::handlers::room::list())
}

/// POST /room
fn create(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("room")
        .and(warp::post())
        .and_then(move || crate::handlers::room::create())
}
