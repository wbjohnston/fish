use crate::models::user::UserId;
use uuid::Uuid;

use super::deck::{create_deck_transaction, DeckId};

pub type GameId = Uuid;

pub type Chips = i32;

pub type SeatNumber = i32;

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub id: GameId,
    pub name: String,
    pub owner_id: UserId,
    pub deck_id: DeckId,
    pub button_seat_number: SeatNumber,
    pub active_seat_number: SeatNumber,
    pub pot: Chips,
}

pub async fn list_games(db: crate::Db) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    sqlx::query_as!(Game, "SELECT * FROM games")
        .fetch_all(&db)
        .await
        .map(Ok)
        .unwrap()
}

pub async fn fetch_game(db: crate::Db, id: GameId) -> Result<Game, Box<dyn std::error::Error>> {
    sqlx::query_as!(Game, "SELECT * FROM games WHERE id = $1", id)
        .fetch_one(&db)
        .await
        .map(Ok)
        .unwrap()
}

pub async fn create_game(
    db: crate::Db,
    name: String,
    owner_id: UserId,
) -> Result<Game, Box<dyn std::error::Error>> {
    let tx = db.begin().await.unwrap();
    let (mut tx, deck_id) = create_deck_transaction(tx).await.unwrap();

    let game = sqlx::query_as!(
        Game,
        "INSERT INTO games (name, owner_id, deck_id) VALUES ($1, $2, $3) RETURNING *",
        name,
        owner_id,
        deck_id
    )
    .fetch_one(&mut tx)
    .await
    .unwrap();

    tx.commit().await.unwrap();

    Ok(game)
}
