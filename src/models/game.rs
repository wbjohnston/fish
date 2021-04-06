use crate::models::user::UserId;
use uuid::Uuid;

use super::{
    card::Card,
    deck::{create_deck_transaction, DeckId},
    hand::HandId,
};

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
    pub status: String,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct GameSession {
    pub user_id: UserId,
    pub game_id: GameId,
    pub stack: Chips,
    pub hand_id: HandId,
    pub seat_number: SeatNumber,
    pub status: String,
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

pub async fn deal_cards_to_players(
    db: crate::Db,
    game_id: GameId,
) -> Result<Vec<(UserId, Vec<Card>)>, Box<dyn std::error::Error>> {
    /*
    0: 0 6
    1: 1 7
    2: 2 8
    3: 3 9
    4: 4 10
    5: 5 11
    */
    todo!()
}

pub async fn get_players_hand(
    db: crate::Db,
    game_id: GameId,
    user_id: UserId,
) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let cards = sqlx::query_as!(Card, r#"
        SELECT card_to_deck.id, card_to_deck.value, card_to_deck.suit FROM hands
        JOIN players ON players.hand_id = hands.id
        JOIN users ON users.id = players.user_id 
        JOIN card_to_deck ON card_to_deck.id = hands.first_card_id OR card_to_deck.id = hands.second_card_id
        WHERE users.id = $1 AND players.game_id = $2
    "#, user_id, game_id).fetch_all(&db).await.unwrap();

    Ok(cards)
}

pub async fn join_game(
    db: crate::Db,
    game_id: GameId,
    user_id: UserId,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}

pub async fn leave_game(
    db: crate::Db,
    game_id: GameId,
    user_id: UserId,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}
