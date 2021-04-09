use crate::{
    models::{game::Game, notification::Notification},
    prelude::*,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};

use tracing::*;
use uuid::Uuid;
use warp::{ws::Message as WsMessage, Reply};

use crate::{
    models::{
        game::get_table,
        game::Table,
        game::{Chips, GameId, SeatNumber},
        session::Session,
        user::{SanitizedUser, User, UserId},
    },
    services::auth::hash_password,
};

pub async fn list(db: Db) -> WebResult<impl warp::Reply> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&db)
        .await
        .unwrap();

    let sanitized: Vec<_> = users.into_iter().map(SanitizedUser::from).collect();

    Ok(warp::reply::json(&sanitized))
}

pub async fn create(db: Db, new_user: crate::models::user::NewUser) -> WebResult<impl Reply> {
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

pub async fn fetch(db: Db, id: Uuid) -> WebResult<impl warp::Reply> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1 LIMIT 1", id)
        .fetch_one(&db)
        .await
        .unwrap();

    let sanitized = SanitizedUser::from(user);

    Ok(warp::reply::json(&sanitized))
}

pub async fn ws(
    context: Context,
    session: Session,
    _id: UserId,
    ws: warp::ws::Ws,
) -> WebResult<impl warp::Reply> {
    // TODO(will): kill connection if a session is killed
    Ok(ws.on_upgrade(move |socket| async {
        let (mut sink, mut stream) = socket.split();
        let (tx, mut rx) = tokio::sync::mpsc::channel(32);

        let mut listener = sqlx::postgres::PgListener::connect_with(&context.db).await.unwrap();
        listener.listen("game_notifications").await.unwrap();

        let rx_handle = tokio::spawn(async move {
            loop {

                tokio::select! {
                    Some(Ok(x)) = stream.next() => {
                        if let Some(_) = x.close_frame() {
                            info!("websocket connection closed");

                            return;
                        }

                        let message: Message =
                            serde_json::from_str(x.to_str().expect("unable to convert to str"))
                                .expect("unable to deserialize message");

                        match message.action {
                            Action::Join => {
                                Game::join_game(context.db.clone(), message.game_id, session.owner_id)
                                    .await
                                    .unwrap();
                            }
                            Action::Bet { amount } => {
                                Game::accept_player_move(
                                    context.db.clone(),
                                    message.game_id,
                                    session.owner_id,
                                    message.action.clone(),
                                )
                                .await
                                .unwrap();
                            }
                            Action::Leave => {
                                Game::leave_game(context.db.clone(), message.game_id, session.owner_id)
                                    .await
                                    .unwrap();
                            }
                            Action::Fold => {
                                Game::accept_player_move(
                                    context.db.clone(),
                                    message.game_id,
                                    session.owner_id,
                                    message.action.clone(),
                                )
                                .await
                                .unwrap();
                                // Game::fold_player(db.clone(), message.game_id, session.owner_id)
                                //     .await
                                //     .unwrap();
                            }
                            Action::Stand => {
                                Game::stand_player(context.db.clone(), message.game_id, session.owner_id)
                                    .await
                                    .unwrap();
                            }
                            Action::Sit { seat_number: None } => {
                                Game::sit_player_at_first_available_seat(
                                    context.db.clone(),
                                    message.game_id,
                                    session.owner_id,
                                )
                                .await
                                .unwrap();
                            }
                            Action::Sit {
                                seat_number: Some(x),
                            } => {
                                Game::sit_player_at_seat(
                                    context.db.clone(),
                                    message.game_id,
                                    session.owner_id,
                                    x,
                                )
                                .await
                                .unwrap();
                            }
                            Action::Sync => {
                                let table = get_table(context.db.clone(), message.game_id)
                                    .await
                                    .unwrap();
                                tx.send(Event::Update {
                                    game_id: message.game_id,
                                })
                                .await
                                .unwrap();
                            }
                            Action::Start => {
                                Game::start_game(context.db.clone(), message.game_id)
                                    .await
                                    .unwrap();
                            }
                            Action::Pause => {
                                Game::pause_game(context.db.clone(), message.game_id)
                                    .await
                                    .unwrap();
                            }
                        }

                        let response = Event::Acknowledge { message };

                        tx.send(response).await.expect("failed to send to sender");
                    }
                    Ok(msg) = listener.recv() => {
                        let notif: Notification = serde_json::from_str(msg.payload()).unwrap();

                        tx.send(Event::Update {
                            game_id: notif.game_id
                        }).await.unwrap();
                    }
                }
        }
        });

        let tx_handle = tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                dbg!(&event);
                sink.send(WsMessage::text(serde_json::to_string(&event).unwrap()))
                    .await
                    .unwrap();
            }
        });

        tokio::try_join!(rx_handle, tx_handle).unwrap();
    }))
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub game_id: GameId,
    pub action: Action,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload")]
#[serde(rename_all = "camelCase")]
pub enum Event {
    /// A new game was created
    NewGame {
        game_id: GameId,
    },
    /// A game's state has been updated
    Update {
        game_id: GameId,
        // table: Table,
    },
    /// Acknowledging a user command
    Acknowledge {
        message: Message,
    },
    Error {},
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(tag = "kind", content = "options")]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Sync,
    Start,
    Pause,
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
