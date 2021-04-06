pub type CardId = uuid::Uuid;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Suit {
    Diamonds,
    Spades,
    Clubs,
    Hearts,
}

pub static SUITS: phf::Map<&'static str, Suit> = phf::phf_map! {
    "diamonds" => Suit::Diamonds,
    "spades" => Suit::Spades,
    "clubs" => Suit::Clubs,
    "hearts" => Suit::Hearts,
};

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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

pub static VALUES: phf::Map<&'static str, Value> = phf::phf_map! {
    "two" => Value::Two,
    "three" => Value::Three,
    "four" => Value::Four,
    "five" => Value::Five,
    "six" => Value::Six,
    "seven" => Value::Seven,
    "eight" => Value::Eight,
    "nine" => Value::Nine,
    "ten" => Value::Ten,
    "jack" => Value::Jack,
    "queen" => Value::Queen,
    "king" => Value::King,
    "ace" => Value::Ace,
};

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Card {
    pub value: String,
    pub suit: String,
}
