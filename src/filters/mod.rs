mod client;
mod room;
mod user;

use warp::Filter;

pub fn index(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let index = warp::path::end().map(|| warp::reply::html("not found"));

    index
        .or(client::index(db.clone()))
        .or(room::index(db.clone()))
        .or(user::index(db.clone()))
}
