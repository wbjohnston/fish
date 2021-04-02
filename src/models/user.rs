use serde::{Deserialize, Serialize};
// use sqlx::types::Uuid;
use sqlx::types::uuid::Uuid;

pub type UserId = Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
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
    pub id: Uuid,
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
