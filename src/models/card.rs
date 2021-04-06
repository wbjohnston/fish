pub type CardId = uuid::Uuid;

pub const SUITS: [&str; 4] = ["diamonds", "spades", "clubs", "hearts"];
pub const VALUES: [&str; 12] = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K"];

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Card {
    pub id: CardId,
    pub value: String,
    pub suit: String,
}
