use sqlx::types::Uuid;

use crate::models::user::UserId;

pub type ClientId = Uuid;
pub type ClientSecret = Uuid;

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Client {
    pub id: ClientId,

    pub name: String,

    pub owner_id: UserId,

    pub client_secret: ClientSecret,
}
