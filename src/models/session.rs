use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user::UserId;

pub type SessionId = Uuid;

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub owner_id: UserId,
}
