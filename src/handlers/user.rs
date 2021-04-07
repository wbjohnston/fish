use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;


use uuid::Uuid;
use warp::{ws::Message as WsMessage, Reply};

use crate::{models::game::join_game, models::game::leave_game, models::game::sit_player_at_first_available_seat, models::game::sit_player_at_seat, models::game::stand_player, models::{game::Table, game::bet_player, game::deal_flop, game::deal_turn, game::fold_player, game::get_table, game::{Chips, GameId, SeatNumber}, session::Session, game::deal_river, user::{SanitizedUser, User, UserId}}, services::auth::hash_password};

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
    // TODO(will): kill connection if a session is killed
    Ok(ws.on_upgrade(move |socket| async {
        let (mut sink, mut stream) = socket.split();
        let (tx, mut rx) = tokio::sync::mpsc::channel(32);

        let rx_handle = tokio::spawn(async move {
            while let Some(Ok(x)) = stream.next().await {
                dbg!(&x);
                let message: Message =
                    serde_json::from_str(x.to_str().expect("unable to convert to str"))
                        .expect("unable to deserialize message");

                match message.action {
                    Action::Join => {
                        join_game(db.clone(), message.game_id, session.owner_id)
                            .await
                            .unwrap();
                    }
                    Action::Bet { amount } => {
                        bet_player(db.clone(), message.game_id, session.owner_id, amount)
                            .await
                            .unwrap();
                    }
                    Action::Leave => {
                        leave_game(db.clone(), message.game_id, session.owner_id)
                            .await
                            .unwrap();
                    }
                    Action::Fold => {
                        fold_player(db.clone(), message.game_id, session.owner_id)
                            .await
                            .unwrap();
                    }
                    Action::Stand => {
                        stand_player(db.clone(), message.game_id, session.owner_id)
                            .await
                            .unwrap();
                    }
                    Action::Sit { seat_number: None } => {
                        sit_player_at_first_available_seat(
                            db.clone(),
                            message.game_id,
                            session.owner_id,
                        )
                        .await
                        .unwrap();
                        deal_flop(db.clone(), message.game_id).await.unwrap();
                        deal_turn(db.clone(), message.game_id).await.unwrap();
                        deal_river(db.clone(), message.game_id).await.unwrap();
                    }
                    Action::Sit {
                        seat_number: Some(x),
                    } => {
                        sit_player_at_seat(db.clone(), message.game_id, session.owner_id, x)
                            .await
                            .unwrap();
                    }
                    Action::Sync => {
                        let table = get_table(db.clone(), message.game_id).await.unwrap();
                        tx.send(Event::Update {
                            game_id: message.game_id,
                            table,
                        })
                        .await
                        .unwrap();
                    }
                }

                let response = Event::Acknowledge { message };

                tx.send(response).await.expect("failed to send to sender");
            }
        });

        let tx_handle = tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                sink.send(WsMessage::text(serde_json::to_string(&event).unwrap()))
                    .await
                    .unwrap();
            }
        });

        tokio::try_join!(rx_handle, tx_handle).unwrap();
    }))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub game_id: GameId,
    pub action: Action,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Event {
    /// A new game was created
    NewGame {
        game_id: GameId,
    },
    /// A game's state has been updated
    Update {
        game_id: GameId,
        table: Table,
    },
    /// Acknowledging a user command
    Acknowledge {
        message: Message,
    },
    Error {},
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "options")]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Sync,
    Stand,
    Fold,
    Leave,
    Join,
    Bet {
        amount: Chips,
    },
    Sit {
        /// The seat to to sit down at. If not specified the first available seat will be taken
        seat_number: Option<SeatNumber>,
    },
}
