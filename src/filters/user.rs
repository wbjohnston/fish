use crate::prelude::*;
use uuid::Uuid;
use warp::Filter;

pub fn index(
    context: Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(context.clone())
        .or(create(context.clone()))
        .or(fetch(context))
}

fn list(
    context: Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::get())
        .and_then(move || crate::handlers::user::list(context.db.clone()))
}

fn create(
    context: Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |user: crate::models::user::NewUser| {
            crate::handlers::user::create(context.db.clone(), user)
        })
}

fn fetch(
    context: Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user" / Uuid)
        .and(warp::get())
        .and_then(move |id| crate::handlers::user::fetch(context.db.clone(), id))
}
