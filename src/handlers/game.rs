use crate::{
    models::{
        game::{Game, GameId},
        session::Session,
    },
    prelude::*,
};

pub async fn list(db: Db) -> WebResult<impl warp::Reply> {
    let games = Game::list(db).await.unwrap();

    Ok(warp::reply::json(&games))
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NewGameRequest {
    pub name: String,
}

pub async fn create(
    db: Db,
    session: Session,
    new_game: NewGameRequest,
) -> WebResult<impl warp::Reply> {
    let game = Game::create(db.clone(), new_game.name, session.owner_id)
        .await
        .unwrap();

    Ok(warp::reply::with_status(
        warp::reply::json(&game),
        warp::http::StatusCode::OK,
    ))
}

pub async fn fetch(db: Db, id: GameId) -> WebResult<impl warp::Reply> {
    let game = Game::fetch(db.clone(), id).await.unwrap();

    Ok(warp::reply::with_status(
        warp::reply::json(&game),
        warp::http::StatusCode::OK,
    ))
}
