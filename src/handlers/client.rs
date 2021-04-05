use crate::models::{
    client::{Client, ClientId, SanitizedClient},
    game::{Chips, GameId, SeatNumber},
    session::Session,
    user::{user_owns_client, UserId},
};
use futures::{SinkExt, StreamExt};
use std::convert::Infallible;
use warp::ws::Message;

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
    Ok(ws.on_upgrade(move |socket| async {
        let (mut sink, mut stream) = socket.split();

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

pub async fn list(db: crate::Db, session: Session) -> Result<impl warp::Reply, Infallible> {
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
