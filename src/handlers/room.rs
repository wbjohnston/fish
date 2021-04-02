use tokio::sync::Mutex;

use std::convert::Infallible;
use std::{collections::HashMap, sync::Arc};
use tracing::*;

pub async fn list() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("ok"))
}

pub async fn create() -> Result<impl warp::Reply, Infallible> {
    info!("creating room");
    // let (tx, rx) = tokio::sync::mpsc::channel(32);

    // let room = Room {
    //     // TODO(will): need to generate ids on the fly
    //     id: 0,
    //     table: Table::default(),
    //     client_connections: HashMap::new(),
    //     tx,
    //     rx,
    // };

    // tokio::spawn(room.run());

    Ok(warp::reply::html("<h1>hello</h1>"))
}
