use tokio::sync::Mutex;

use std::convert::Infallible;
use std::{collections::HashMap, sync::Arc};
use tracing::*;

use crate::models::{room::Room, session::Session, user::UserId};

pub async fn list(db: crate::Db, _session: Session) -> Result<impl warp::Reply, Infallible> {
    let rooms = sqlx::query_as!(Room, "SELECT * FROM rooms")
        .fetch_all(&db)
        .await
        .unwrap();

    Ok(warp::reply::json(&rooms))
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NewRoomRequest {
    pub name: String,
}

pub async fn create(
    db: crate::Db,
    session: Session,
    new_room: NewRoomRequest,
) -> Result<impl warp::Reply, Infallible> {
    let room = sqlx::query_as!(
        Room,
        "INSERT INTO rooms (name, owner_id) VALUES ($1, $2) RETURNING *",
        new_room.name,
        session.owner_id
    )
    .fetch_one(&db)
    .await
    .unwrap();

    Ok(warp::reply::with_status(
        warp::reply::json(&room),
        warp::http::StatusCode::OK,
    ))
}
