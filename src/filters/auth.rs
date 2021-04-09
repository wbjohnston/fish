use crate::prelude::*;
use warp::Filter;

use crate::{
    handlers::auth::LoginRequest,
    models::{session::Session, session::SessionId, user::NewUser},
};

pub fn authorization_token_filter(
    context: Context,
) -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header::optional("Authorization"))
        .and(warp::cookie::optional("authorization"))
        .and_then(
            move |header: Option<SessionId>, cookie: Option<SessionId>| {
                crate::handlers::auth::authorize(context.db.clone(), header, cookie)
            },
        )
}

pub fn index(
    context: Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    login(context.clone())
        .or(register(context.clone()))
        .or(logout(context))
}

pub fn login(
    context: Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("auth" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |login_req: LoginRequest| {
            crate::handlers::auth::login(context.db.clone(), login_req)
        })
}

pub fn register(
    context: Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // post user
    warp::path!("auth" / "register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |new_user: NewUser| {
            crate::handlers::user::create(context.db.clone(), new_user)
        })
}

pub fn logout(
    context: Context,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("auth" / "logout")
        .and(warp::get())
        .and(authorization_token_filter(context.clone()))
        .and_then(move |session| crate::handlers::auth::logout(context.db.clone(), session))
}
