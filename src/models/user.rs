use serde::{Deserialize, Serialize};
use sqlx::types::uuid::Uuid;

pub type UserId = Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub role: String,
    pub password_hash: String,
}

impl User {
    pub fn role(&self) -> Role {
        parse_role(self.role.as_str()).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SanitizedUser {
    pub id: UserId,
    pub role: Role,
    pub username: String,
}

impl From<User> for SanitizedUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            role: parse_role(user.role.as_str()).unwrap(),
            username: user.username,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    User,
    Admin,
}

static ROLES: phf::Map<&'static str, Role> = phf::phf_map! {
    "user" => Role::User,
    "admin" => Role::Admin,
};

pub fn parse_role(role: &str) -> Option<Role> {
    ROLES.get(role).cloned()
}
