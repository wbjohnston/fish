use crate::models::{
    client::{Client, ClientId},
    session::Session,
    user::UserId,
};
use futures::{SinkExt, StreamExt};
use std::convert::Infallible;
use tracing::*;
use warp::Filter;

pub async fn ws(db: crate::Db, id: ClientId, socket: warp::ws::WebSocket) {
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

pub async fn list(db: crate::Db, _session: Session) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("<h1>hello</h1>"))
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NewClientRequest {
    pub name: String,
    pub owner_id: UserId,
}

pub async fn create(
    db: crate::Db,
    _session: Session,
    new_client: NewClientRequest,
) -> Result<impl warp::Reply, Infallible> {
    let client = sqlx::query_as!(
        crate::models::client::Client,
        "INSERT INTO clients (name, owner_id) VALUES ($1, $2) RETURNING *",
        new_client.name,
        new_client.owner_id
    )
    .fetch_one(&db)
    .await
    .unwrap();

    info!(?client, "created client");

    Ok(warp::reply::json(&client))
}

pub async fn fetch(
    db: crate::Db,
    _session: Session,
    id: ClientId,
) -> Result<impl warp::Reply, Infallible> {
    let client = match sqlx::query_as!(Client, "SELECT * FROM clients WHERE id = $1", id)
        .fetch_one(&db)
        .await
    {
        Ok(client) => client,
        Err(_) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&()),
                warp::http::StatusCode::NOT_FOUND,
            ));
        }
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&client),
        warp::http::StatusCode::OK,
    ))
}
