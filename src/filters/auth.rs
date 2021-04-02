use warp::Filter;

use crate::{handlers::auth::LoginRequest, models::user::NewUser};

pub fn index(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    login(db.clone()).or(register(db.clone()))
}

pub fn login(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("auth" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |login_req: LoginRequest| {
            crate::handlers::auth::login(db.clone(), login_req)
        })
}

pub fn register(
    db: crate::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // post user
    warp::path!("auth" / "register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |new_user: NewUser| crate::handlers::user::create(db.clone(), new_user))
}
