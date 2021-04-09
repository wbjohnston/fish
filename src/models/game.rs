use crate::models::user::UserId;
use serde::{Deserialize, Serialize};
use tracing::*;
use uuid::Uuid;
use crate::prelude::*;

use super::{
    card::{Card, CardId},
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
    pub last_to_bet_seat_number: Option<SeatNumber>,
    pub flop_1_card_id: Option<CardId>,
    pub flop_2_card_id: Option<CardId>,
    pub flop_3_card_id: Option<CardId>,
    pub turn_card_id: Option<CardId>,
    pub river_card_id: Option<CardId>,
    pub pot: Chips,
    pub status: String,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct GameSession {
    pub user_id: UserId,
    pub game_id: GameId,
    pub stack: Option<Chips>,
    pub bet: Option<Chips>,
    pub hand_id: Option<HandId>,
    pub seat_number: Option<SeatNumber>,
    pub status: String,
}

pub async fn list_games(db: Db) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    sqlx::query_as!(Game, "SELECT * FROM games")
        .fetch_all(&db)
        .await
        .map(Ok)
        .unwrap()
}

pub async fn fetch_game(db: Db, id: GameId) -> Result<Game, Box<dyn std::error::Error>> {
    sqlx::query_as!(Game, "SELECT * FROM games WHERE id = $1", id)
        .fetch_one(&db)
        .await
        .map(Ok)
        .unwrap()
}

pub async fn create_game(
    db: Db,
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
    db: Db,
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
    db: Db,
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
    db: Db,
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

pub async fn deal_flop(db: Db, game_id: GameId) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = db.begin().await.unwrap();

    sqlx::query!(
        r#"
            with current_deck as (
                select * from decks where id = (select deck_id from games where id = $1)
            )
            update games
                set
                    flop_1_card_id = (select id from card_to_deck where deck_id = games.deck_id AND position = (select position from current_deck) + 1),
                    flop_2_card_id = (select id from card_to_deck where deck_id = games.deck_id AND position = (select position from current_deck) + 2),
                    flop_3_card_id = (select id from card_to_deck where deck_id = games.deck_id AND position = (select position from current_deck) + 3)
            where
                id = $1
        "#,
        game_id
    ).execute(&mut tx).await.unwrap();

    sqlx::query!(
        r#"
            update decks
                set
                    position = position + 4
            where id = (select deck_id from games where id = $1)
        "#,
        game_id
    )
    .execute(&mut tx)
    .await
    .unwrap();

    tx.commit().await.unwrap();

    Ok(())
}

pub async fn deal_turn(db: Db, game_id: GameId) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = db.begin().await.unwrap();

    debug!("IN HERE");

    sqlx::query!(
        r#"
            with current_deck as (
                select * from decks where id = (select deck_id from games where id = $1)
            )
            update games
                set
                    turn_card_id = (select id from card_to_deck where deck_id = games.deck_id AND position = (select position from current_deck) + 1)
            where
                id = $1
        "#,
        game_id
    ).execute(&mut tx).await.unwrap();

    sqlx::query!(
        r#"
            update decks
                set
                    position = position + 2
            where id = (select deck_id from games where id = $1)
        "#,
        game_id
    )
    .execute(&mut tx)
    .await
    .unwrap();

    tx.commit().await.unwrap();

    Ok(())
}

pub async fn deal_river(db: Db, game_id: GameId) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = db.begin().await.unwrap();

    debug!("IN HERE");

    sqlx::query!(
        r#"
            with current_deck as (
                select * from decks where id = (select deck_id from games where id = $1)
            )
            update games
                set
                    river_card_id = (select id from card_to_deck where deck_id = games.deck_id AND position = (select position from current_deck) + 1)
            where
                id = $1
        "#,
        game_id
    ).execute(&mut tx).await.unwrap();

    sqlx::query!(
        r#"
            update decks
                set
                    position = position + 2
            where id = (select deck_id from games where id = $1)
        "#,
        game_id
    )
    .execute(&mut tx)
    .await
    .unwrap();

    tx.commit().await.unwrap();
    Ok(())
}

pub async fn fold_player(
    db: Db,
    game_id: GameId,
    user_id: UserId,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}

pub async fn bet_player(
    db: Db,
    game_id: GameId,
    user_id: UserId,
    amount: Chips,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}

pub async fn deal_cards_to_players(
    db: Db,
    game_id: GameId,
) -> Result<Vec<(UserId, Vec<Card>)>, Box<dyn std::error::Error>> {
    // sqlx::query!(
    //     r#"
    //         with current_players as (
    //             select * from players where game_id = $1
    //         )
    //         update players
    //             set

    //         where
    //             game_id=$1
    //     "#,
    //     game_id
    // )
    // .execute(&context.db)
    // .await
    // .unwrap();

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

pub async fn remove_community_cards<'a>(
    mut tx: sqlx::Transaction<'a, sqlx::Postgres>,
    game_id: GameId,
) -> Result<sqlx::Transaction<'a, sqlx::Postgres>, Box<dyn std::error::Error>> {
    sqlx::query!(
        r#"
            update games
            set
                flop_1_card_id = null, 
                flop_2_card_id = null, 
                flop_3_card_id = null, 
                turn_card_id = null, 
                river_card_id = null
            where
                id = $1
        "#,
        game_id
    )
    .execute(&mut tx)
    .await?;
    Ok(tx)
}

pub async fn shuffle_game_deck_transaction<'a>(
    mut tx: sqlx::Transaction<'a, sqlx::Postgres>,
    game_id: GameId,
) -> Result<sqlx::Transaction<'a, sqlx::Postgres>, Box<dyn std::error::Error>> {
    let deck_id = sqlx::query_scalar!("select deck_id from games where id = $1", game_id)
        .fetch_one(&mut tx)
        .await?;

    let tx = super::deck::shuffle_deck_transaction(tx, deck_id).await?;

    Ok(tx)
}

pub async fn remove_player_cards<'a>(
    mut tx: sqlx::Transaction<'a, sqlx::Postgres>,
    game_id: GameId,
) -> Result<sqlx::Transaction<'a, sqlx::Postgres>, Box<dyn std::error::Error>> {
    sqlx::query!(
        r#"
            delete from hands where id in (select hand_id from players where game_id=$1)
        "#,
        game_id
    )
    .execute(&mut tx)
    .await?;

    Ok(tx)
}

pub async fn distribute_winnings<'a>(
    mut tx: sqlx::Transaction<'a, sqlx::Postgres>,
    game_id: GameId,
) -> Result<sqlx::Transaction<'a, sqlx::Postgres>, Box<dyn std::error::Error>> {
    todo!()
}

pub async fn round_is_over(
    db: Db,
    game_id: GameId,
) -> Result<bool, Box<dyn std::error::Error>> {
    todo!()
}

pub async fn player_is_active_player(
    db: Db,
    game_id: GameId,
    user_id: UserId,
) -> Result<bool, Box<dyn std::error::Error>> {
    let maybe_active_player = sqlx::query_as!(
        GameSession, 
        r#"
            select * from players where game_id = $1 AND user_id = $2 and seat_number = (select active_seat_number from games where id = $1)
        "#,
        game_id,
        user_id
    ).fetch_optional(&db).await?;

    match maybe_active_player {
        Some(player) if player.user_id == user_id => Ok(true),
        _ => Ok(false)
    }
}

pub async fn player_is_last_to_act(
    db: Db,
    game_id: GameId,
    user_id: UserId,
) -> Result<bool, Box<dyn std::error::Error>> {
    todo!()
}

pub async fn advance_button<'a>(
    tx: sqlx::Transaction<'a, sqlx::Postgres>,
    game_id: GameId,
) -> Result<sqlx::Transaction<'a, sqlx::Postgres>, Box<dyn std::error::Error>> {
    todo!()
}

pub async fn game_can_continue<'a>(
    db: Db,
    game_id: GameId,
) -> Result<bool, Box<dyn std::error::Error>> {
    todo!()
}

pub async fn end_round(db: Db, game_id: GameId) -> Result<(), Box<dyn std::error::Error>> {
    /*
    1. delete community cards
    2. delete player cards
    3. write to stats table
    4. reshuffle cards
    5. distribute pot to winner
    6. advance button
    */
    let tx = db.begin().await.unwrap();

    let tx = remove_community_cards(tx, game_id).await?;
    let tx = remove_player_cards(tx, game_id).await?;
    let tx = shuffle_game_deck_transaction(tx, game_id).await?;
    let tx = distribute_winnings(tx, game_id).await?;
    let tx = advance_button(tx, game_id).await?;

    tx.commit().await.unwrap();

    if game_can_continue(db, game_id).await? {
        todo!()
    } else {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub cards: Vec<Card>,
    pub players: Vec<Option<Player>>,
    pub pot: Chips,
    pub active_seat_number: SeatNumber,
    pub button_seat_number: SeatNumber,
    pub status: GameStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: UserId,
    pub bet: Option<Chips>,
    pub hand: Option<Vec<Card>>,
    pub stack: Chips,
    pub status: PlayerStatus,
}

pub async fn get_table(
    _db: Db,
    _game_id: GameId,
) -> Result<Table, Box<dyn std::error::Error>> {
    todo!()
}

pub async fn join_game(
    db: Db,
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
    db: Db,
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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub static GAME_STATUSES: phf::Map<&'static str, GameStatus> = phf::phf_map! {
    "created" => GameStatus::Created,
    "running" => GameStatus::Running,
    "ended" => GameStatus::Ended,
    "paused" => GameStatus::Paused,
};


#[derive(Debug, Serialize, Deserialize)]
pub enum GamePhase {
    PreFlop,
    Flop,
    Turn,
    River,
    End,
}

#[allow(dead_code)]
pub static GAME_PHASES: phf::Map<&'static str, GamePhase> = phf::phf_map! {
    "preFlop" => GamePhase::PreFlop,
    "flop" => GamePhase::Flop,
    "turn" => GamePhase::Turn,
    "river" => GamePhase::River,
    "end" => GamePhase::End,
};
