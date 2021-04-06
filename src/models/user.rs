use serde::{Deserialize, Serialize};
use sqlx::types::uuid::Uuid;

use super::{game::GameId, hand::Hand};

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
