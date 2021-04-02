use tokio::sync::Mutex;

use crate::types::*;
use std::convert::Infallible;
use std::{collections::HashMap, sync::Arc};
use tracing::*;

use crate::poker::Table;

pub async fn list(state: Arc<Mutex<State>>) -> Result<impl warp::Reply, Infallible> {
    let rooms: Vec<_> = {
        let lock = state.lock().await;
        lock.rooms.iter().map(|x| format!("{:?}", x)).collect()
    };

    Ok(warp::reply::html(format!("{:?}", rooms)))
}

pub async fn create(state: Arc<Mutex<State>>) -> Result<impl warp::Reply, Infallible> {
    info!("creating room");
    let (tx, rx) = tokio::sync::mpsc::channel(32);

    state.lock().await.rooms.insert(0, tx.clone());

    let room = Room {
        // TODO(will): need to generate ids on the fly
        id: 0,
        table: Table::default(),
        client_connections: HashMap::new(),
        tx,
        rx,
    };

    tokio::spawn(room.run());

    Ok(warp::reply::html("<h1>hello</h1>"))
}
