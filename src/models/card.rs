use super::deck::DeckId;

pub type CardId = uuid::Uuid;

pub const SUITS: [&str; 4] = ["diamonds", "spades", "clubs", "hearts"];
pub const VALUES: [&str; 12] = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K"];

#[derive(Debug, sqlx::FromRow)]
pub struct Card {
    pub id: CardId,
    pub deck_id: DeckId,
    pub position: i32,
    pub value: String,
    pub suit: String,
}
