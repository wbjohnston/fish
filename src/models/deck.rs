pub type DeckId = uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Deck {
    pub id: DeckId,
    pub position: i32,
}
