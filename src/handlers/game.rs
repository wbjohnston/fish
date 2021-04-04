use tokio::sync::Mutex;

use std::convert::Infallible;
use std::{collections::HashMap, sync::Arc};
use tracing::*;

use crate::models::{game::Game, session::Session, user::UserId};

pub async fn list(db: crate::Db, _session: Session) -> Result<impl warp::Reply, Infallible> {
    let games = sqlx::query_as!(Game, "SELECT * FROM games")
        .fetch_all(&db)
        .await
        .unwrap();

    Ok(warp::reply::json(&games))
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NewGameRequest {
    pub name: String,
}

pub async fn create(
    db: crate::Db,
    session: Session,
    new_game: NewGameRequest,
) -> Result<impl warp::Reply, Infallible> {
    let game = sqlx::query_as!(
        Game,
        "INSERT INTO games (name, owner_id) VALUES ($1, $2) RETURNING *",
        new_game.name,
        session.owner_id
    )
    .fetch_one(&db)
    .await
    .unwrap();

    Ok(warp::reply::with_status(
        warp::reply::json(&game),
        warp::http::StatusCode::OK,
    ))
}
