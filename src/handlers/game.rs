use std::convert::Infallible;

use crate::models::{
    game::create_game, game::fetch_game, game::list_games, game::GameId, session::Session,
};

pub async fn list(db: crate::Db) -> Result<impl warp::Reply, Infallible> {
    let games = list_games(db).await.unwrap();

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
    let game = create_game(db.clone(), new_game.name, session.owner_id)
        .await
        .unwrap();

    Ok(warp::reply::with_status(
        warp::reply::json(&game),
        warp::http::StatusCode::OK,
    ))
}

pub async fn fetch(db: crate::Db, id: GameId) -> Result<impl warp::Reply, Infallible> {
    let game = fetch_game(db.clone(), id).await.unwrap();

    Ok(warp::reply::with_status(
        warp::reply::json(&game),
        warp::http::StatusCode::OK,
    ))
}
