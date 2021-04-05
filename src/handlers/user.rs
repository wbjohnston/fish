use futures::{SinkExt, StreamExt};
use std::convert::Infallible;
use tracing::*;

use uuid::Uuid;
use warp::{ws::Message, Reply};

use crate::{
    models::{
        game::{Chips, GameId, SeatNumber},
        session::Session,
        user::{SanitizedUser, User, UserId},
    },
    services::auth::hash_password,
};

pub async fn list(db: crate::Db) -> Result<impl warp::Reply, Infallible> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&db)
        .await
        .unwrap();

    let sanitized: Vec<_> = users.into_iter().map(SanitizedUser::from).collect();

    Ok(warp::reply::json(&sanitized))
}

pub async fn create(
    db: crate::Db,
    new_user: crate::models::user::NewUser,
) -> Result<impl Reply, Infallible> {
    let hash = hash_password(new_user.password.as_bytes());

    let user: User = sqlx::query_as!(
        User,
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING *",
        new_user.username,
        hash,
    )
    .fetch_one(&db)
    .await
    .unwrap();

    let sanitized = SanitizedUser::from(user);

    Ok(warp::reply::json(&sanitized))
}

pub async fn fetch(db: crate::Db, id: Uuid) -> Result<impl warp::Reply, Infallible> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1 LIMIT 1", id)
        .fetch_one(&db)
        .await
        .unwrap();

    let sanitized = SanitizedUser::from(user);

    Ok(warp::reply::json(&sanitized))
}

pub async fn ws(
    db: crate::Db,
    session: Session,
    _id: UserId,
    ws: warp::ws::Ws,
) -> Result<impl warp::Reply, Infallible> {
    // TODO(will): verify that the client owns
    // Just echo all messages back...
    Ok(ws.on_upgrade(move |socket| async {
        let (mut sink, mut stream) = socket.split();

        info!("opened connection");

        let (tx, mut rx) = tokio::sync::mpsc::channel(32);

        let rx_handle = tokio::spawn(async move {
            while let Some(Ok(x)) = stream.next().await {
                let command: Command =
                    serde_json::from_str(x.to_str().expect("unable to convert to str"))
                        .expect("unable to deserialize message");

                dbg!(command);

                tx.send(Event::NewState {
                    table: Table {
                        game_id: Default::default(),
                        name: "foobar".to_string(),
                        owner_id: Default::default(),
                        cards: vec![],
                        button_seat_number: 0,
                        active_seat_number: 0,
                        pot: 0,
                        players: vec![],
                    },
                })
                .await
                .unwrap();
            }
        });

        let tx_handle = tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                sink.send(Message::text(serde_json::to_string(&event).unwrap()))
                    .await
                    .unwrap();
            }
        });

        tokio::try_join!(rx_handle, tx_handle).unwrap();
    }))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "options")]
pub enum Command {
    JoinGame(GameId),
    LeaveGame(GameId),
    SubmitMove { game_id: GameId, payload: Move },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "options")]
pub enum Move {
    Fold,
    Bet(Chips),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Event {
    NewState { table: Table },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Table {
    pub game_id: GameId,
    pub name: String,
    pub owner_id: UserId,
    pub cards: Vec<Card>,
    pub button_seat_number: SeatNumber,
    pub active_seat_number: SeatNumber,
    pub pot: Chips,
    pub players: Vec<Player>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub hand: Option<[Card; 2]>,
    pub status: Status,
    pub bet: Chips,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Status {
    Standing,
    Folded,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Card {
    value: Value,
    suit: Suit,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Suit {
    Diamonds,
    Clubs,
    Spades,
    Hearts,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
