use super::client::ClientId;

#[derive(Debug, sqlx::FromRow)]
pub struct Room {
    clients: Vec<ClientId>,
}
