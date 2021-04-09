use crate::prelude::*;
pub type DeckId = uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Deck {
    pub id: DeckId,
    pub position: i32,
}
use crate::models::card::{SUITS, VALUES};
use rand::seq::SliceRandom;

fn generate_deck<'a>() -> impl Iterator<Item = (i32, &'a str, &'a str)> {
    // FIXME(will): is this allocation necessary?
    let suits: Vec<_> = SUITS.keys().collect();
    let values: Vec<_> = VALUES.keys().collect();
    let all_cards = itertools::iproduct!(suits, values);
    let (len, _) = all_cards.size_hint();

    let mut positions: Vec<_> = (0i32..len as i32).collect();

    positions.as_mut_slice().shuffle(&mut rand::thread_rng());
    positions
        .into_iter()
        .zip(all_cards)
        .map(|(position, (&suit, &value))| (position, suit, value))
}

pub async fn shuffle_deck_transaction<'a>(mut tx: Tx<'a>, deck_id: DeckId) -> Result<Tx<'a>> {
    sqlx::query!("DELETE FROM card_to_deck WHERE deck_id = $1", deck_id)
        .execute(&mut tx)
        .await
        .unwrap();

    sqlx::query!("UPDATE decks SET position = 0 WHERE id = $1", deck_id)
        .execute(&mut tx)
        .await
        .unwrap();

    for (position, suit, value) in generate_deck() {
        let _ = sqlx::query!(
            "INSERT INTO card_to_deck (deck_id, position, suit, value) VALUES ($1, $2, $3, $4)",
            deck_id,
            position,
            suit,
            value
        )
        .execute(&mut tx)
        .await
        .unwrap();
    }

    Ok(tx)
}

pub async fn create_deck_transaction<'a>(mut tx: Tx<'a>) -> Result<(Tx<'a>, DeckId)> {
    let deck = sqlx::query_as!(Deck, r#"INSERT INTO decks DEFAULT VALUES RETURNING * "#)
        .fetch_one(&mut tx)
        .await
        .unwrap();

    let cards_iter = generate_deck();

    for (position, suit, value) in cards_iter {
        sqlx::query!(
            "INSERT INTO card_to_deck (deck_id, position, suit, value) VALUES ($1, $2, $3, $4)",
            deck.id,
            position,
            suit,
            value
        )
        .execute(&mut tx)
        .await
        .unwrap();
    }

    Ok((tx, deck.id))
}
