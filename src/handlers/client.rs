use crate::types::*;
use futures::{SinkExt, StreamExt};
use std::convert::Infallible;
use tracing::*;

pub async fn ws(id: RoomId, socket: warp::ws::WebSocket) {
    let (mut tx, mut rx) = socket.split();

    // TODO(will): insert client connection into hashmap
    let (c_tx, mut c_rx) = tokio::sync::mpsc::channel(32);

    let tx_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(x) = c_rx.recv() => {
                    tx.send(x).await.expect("failed to send message to client");

                },
            }
        }
    });

    let rx_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(x) = rx.next() => {
                    debug!("{:?}", x);

                },
            }
        }
    });

    let _ = tokio::try_join!(tx_handle, rx_handle);
}

pub async fn list() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("<h1>hello</h1>"))
}

pub async fn create() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("<h1>hello</h1>"))
}

pub async fn update() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("<h1>hello</h1>"))
}

pub async fn fetch(id: String) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html(format!("<h1>{}</h1>", id)))
}
