use serde::{Deserialize, Serialize};
// use sqlx::types::Uuid;
use crate::models::client::Client;
use sqlx::types::uuid::Uuid;

use super::client::ClientId;

pub type UserId = Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SanitizedUser {
    pub id: UserId,
    pub username: String,
}

impl From<User> for SanitizedUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
        }
    }
}

pub async fn user_owns_client(db: crate::Db, user_id: UserId, client_id: ClientId) -> bool {
    match sqlx::query_as!(
        Client,
        "SELECT * FROM clients where id = $1 AND owner_id = $2",
        client_id,
        user_id
    )
    .fetch_one(&db)
    .await
    {
        Ok(_) => true,
        _ => false,
    }
}
