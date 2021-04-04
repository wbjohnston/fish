use crate::models::user::UserId;
use uuid::Uuid;

pub type GameId = Uuid;

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub id: GameId,
    pub name: String,
    pub owner_id: UserId,
}
