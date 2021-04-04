use super::card::Card;
use uuid::Uuid;

pub type HandId = Uuid;

pub struct Hand {
    id: HandId,
    first_card: Option<Card>,
    second_card: Option<Card>,
}
