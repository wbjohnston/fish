use sqlx::types::Uuid;

use crate::models::user::UserId;

pub type ClientId = Uuid;
pub type ClientSecret = Uuid;

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Client {
    pub id: ClientId,

    pub name: String,

    pub owner_id: UserId,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct SanitizedClient {
    pub id: ClientId,
    pub name: String,
    pub owner_id: UserId,
}

impl From<Client> for SanitizedClient {
    fn from(x: Client) -> Self {
        Self {
            id: x.id,
            name: x.name,
            owner_id: x.owner_id,
        }
    }
}
