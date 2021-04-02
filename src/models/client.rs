use sqlx::types::Uuid;

pub type ClientId = Uuid;
pub type ClientSecret = Uuid;

#[derive(Debug, sqlx::FromRow)]
struct Client {
    id: ClientId,

    client_secret: ClientSecret,
}
