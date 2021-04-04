pub type CardId = uuid::Uuid;

pub struct Card {
    id: CardId,
    position: u32,
    value: Value,
    suit: Suit,
}

pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

pub enum Suit {
    Diamonds,
    Clubs,
    Spades,
    Hearts,
}
