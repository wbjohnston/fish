use crate::models::user::UserId;
use uuid::Uuid;

pub type RoomId = Uuid;

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Room {
    pub id: RoomId,
    pub name: String,
    pub owner_id: UserId,
}
