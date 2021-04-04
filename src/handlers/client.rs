use crate::models::{
    client::{Client, ClientId, SanitizedClient},
    session::Session,
    user::{user_owns_client, UserId},
};
use futures::{FutureExt, SinkExt, StreamExt};
use std::convert::Infallible;
use tracing::*;
use warp::{ws::Message, Filter};

pub async fn ws(
    db: crate::Db,
    session: Session,
    id: ClientId,
    ws: warp::ws::Ws,
) -> Result<impl warp::Reply, Infallible> {
    if !user_owns_client(db.clone(), session.owner_id, id).await {
        // TODO(will): reject unauthorized here
        todo!()
    }

    // TODO(will): verify that the client owns
    // Just echo all messages back...
    Ok(ws.on_upgrade(move |socket| {
        let (tx, rx) = socket.split();
        rx.forward(tx).map(|result| {
            if let Err(e) = result {
                eprintln!("websocket error: {:?}", e);
            }
        })
    }))
}

pub async fn list(db: crate::Db, _session: Session) -> Result<impl warp::Reply, Infallible> {
    let clients = sqlx::query_as!(Client, "SELECT * FROM clients")
        .fetch_all(&db)
        .await
        .unwrap();

    let clients: Vec<_> = clients.into_iter().map(SanitizedClient::from).collect();

    Ok(warp::reply::json(&clients))
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NewClientRequest {
    pub name: String,
}

pub async fn create(
    db: crate::Db,
    session: Session,
    new_client: NewClientRequest,
) -> Result<impl warp::Reply, Infallible> {
    let client = sqlx::query_as!(
        crate::models::client::Client,
        "INSERT INTO clients (name, owner_id) VALUES ($1, $2) RETURNING *",
        new_client.name,
        session.owner_id
    )
    .fetch_one(&db)
    .await
    .unwrap();

    Ok(warp::reply::json(&client))
}

pub async fn fetch(
    db: crate::Db,
    session: Session,
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

    if user_owns_client(db.clone(), session.owner_id, id).await {
        Ok(warp::reply::with_status(
            warp::reply::json(&client),
            warp::http::StatusCode::OK,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&SanitizedClient::from(client)),
            warp::http::StatusCode::OK,
        ))
    }
}
