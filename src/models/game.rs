use crate::models::user::UserId;
use serde::{Deserialize, Serialize};
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
        "INSERT INTO games (name, owner_id, deck_id, status) VALUES ($1, $2, $3, DEFAULT) RETURNING *",
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

pub async fn stand_player(
    db: crate::Db,
    game_id: GameId,
    user_id: UserId,
) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query!(
        r#"
        update players
            set status = 'spectating',
                seat_number = null
            where
                user_id = $1 and
                game_id = $2
        "#,
        user_id,
        game_id
    )
    .execute(&db)
    .await
    .unwrap();

    Ok(())
}

pub async fn sit_player_at_first_available_seat(
    db: crate::Db,
    game_id: GameId,
    user_id: UserId,
) -> Result<SeatNumber, Box<dyn std::error::Error>> {
    let seat = sqlx::query_scalar!(
        r#"
            update players
                set status = 'playing',
                    seat_number = foo.next_seat_number
                        from (select min(bizbaz.seat_number) as next_seat_number from (
                        select generate_series as seat_number from generate_series(0, 5) except
                        select seat_number from players where
                            game_id = $1 and 
                            seat_number is not null
                    ) as bizbaz) as foo
                    where
                        user_id = $2 and seat_number is null and game_id = $1
            returning (seat_number)
        "#,
        game_id,
        user_id
    )
    .fetch_one(&db)
    .await
    .unwrap()
    .unwrap();

    Ok(seat)
}

pub async fn sit_player_at_seat(
    db: crate::Db,
    game_id: GameId,
    user_id: UserId,
    seat_number: SeatNumber,
) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query!(
        r#"
        update players
            set seat_number = $1,
                status = 'playing'
            where
                user_id =  $2 AND
                game_id = $3
        "#,
        seat_number,
        user_id,
        game_id,
    )
    .execute(&db)
    .await
    .unwrap();

    Ok(())
}

pub async fn fold_player(
    db: crate::Db,
    game_id: GameId,
    user_id: UserId,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}

pub async fn bet_player(
    db: crate::Db,
    game_id: GameId,
    user_id: UserId,
    amount: Chips,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!()
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub cards: Vec<Card>,
    pub players: Vec<Option<Player>>,
    pub pot: Chips,
    pub active_seat_number: SeatNumber,
    pub button_seat_number: SeatNumber,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    id: UserId,
    hand: Option<(Card, Card)>,
    stack: Chips,
}

pub async fn get_table(
    db: crate::Db,
    game_id: GameId,
) -> Result<Table, Box<dyn std::error::Error>> {
    todo!()
}

pub async fn get_players_hand(
    db: crate::Db,
    game_id: GameId,
    user_id: UserId,
) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let cards = sqlx::query_as!(Card, r#"
        SELECT card_to_deck.value, card_to_deck.suit FROM hands
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
    sqlx::query!(
        r#"
            INSERT INTO players (
                user_id,
                game_id
            ) VALUES (
                $1,
                $2
            ) ON CONFLICT DO NOTHING
        "#,
        user_id,
        game_id
    )
    .execute(&db)
    .await
    .unwrap();

    // TODO(will): handle the case that the game doesnt exist
    // TODO(will): handle the case that the user doesn't exist

    Ok(())
}

pub async fn leave_game(
    db: crate::Db,
    game_id: GameId,
    user_id: UserId,
) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query!(
        r#"
            DELETE FROM players WHERE user_id = $1 AND game_id = $2
        "#,
        user_id,
        game_id
    )
    .execute(&db)
    .await
    .unwrap();

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlayerStatus {
    Standing,
    Playing,
    Folded,
    Spectating,
}

pub static PLAYER_STATUSES: phf::Map<&'static str, PlayerStatus> = phf::phf_map! {
    "standing" => PlayerStatus::Standing,
    "playing" => PlayerStatus::Playing,
    "folded" => PlayerStatus::Folded,
    "spectating" => PlayerStatus::Spectating,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GameStatus {
    Created,
    Running,
    Ended,
    Paused,
}

pub static GAME_STATUSES: phf::Map<&'static str, GameStatus> = phf::phf_map! {
    "created" => GameStatus::Created,
    "running" => GameStatus::Running,
    "ended" => GameStatus::Ended,
    "paused" => GameStatus::Paused,
};
