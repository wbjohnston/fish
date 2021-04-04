use crate::models::card::{SUITS, VALUES};
use crate::models::{
    card::Card,
    deck::{Deck, DeckId},
};
use rand::seq::SliceRandom;

fn generate_deck<'a>() -> impl Iterator<Item = (i32, &'a str, &'a str)> {
    let all_cards = itertools::iproduct!(SUITS.iter(), VALUES.iter());
    let (len, _) = all_cards.size_hint();

    let mut positions: Vec<_> = (0i32..len as i32).collect();

    positions.as_mut_slice().shuffle(&mut rand::thread_rng());
    positions
        .into_iter()
        .zip(all_cards)
        .map(|(position, (&suit, &value))| (position, suit, value))
}

pub async fn shuffle_deck(
    db: crate::Db,
    deck_id: DeckId,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = db.begin().await.unwrap();

    sqlx::query!("DELETE FROM card_to_deck WHERE deck_id = $1", deck_id)
        .execute(&mut tx)
        .await
        .unwrap();

    sqlx::query!("UPDATE decks SET position = 0 WHERE id = $1", deck_id)
        .execute(&mut tx)
        .await
        .unwrap();

    let mut _cards = vec![];
    for (position, suit, value) in generate_deck() {
        let card = sqlx::query_as!(Card, "INSERT INTO card_to_deck (deck_id, position, suit, value) VALUES ($1, $2, $3, $4) RETURNING *", deck_id, position, suit, value).fetch_one(&mut tx).await.unwrap();
        _cards.push(card);
    }

    tx.commit().await.unwrap();

    Ok(())
}

pub async fn create_deck(db: crate::Db) -> Result<DeckId, Box<dyn std::error::Error>> {
    let mut tx = db.begin().await.unwrap();

    let deck = sqlx::query_as!(Deck, r#"INSERT INTO decks DEFAULT VALUES RETURNING * "#)
        .fetch_one(&mut tx)
        .await
        .unwrap();

    let cards_iter = generate_deck();

    let mut _cards = vec![];
    for (position, suit, value) in cards_iter {
        let card = sqlx::query_as!(Card, "INSERT INTO card_to_deck (deck_id, position, suit, value) VALUES ($1, $2, $3, $4) RETURNING *", deck.id, position, suit, value).fetch_one(&mut tx).await.unwrap();
        _cards.push(card);
    }

    tx.commit().await.unwrap();

    Ok(deck.id)
}

pub async fn draw_next(db: crate::Db, deck_id: DeckId) -> Result<Card, Box<dyn std::error::Error>> {
    let mut tx = db.begin().await.unwrap();

    let card = sqlx::query_as!(
        Card,
        r#"
        SELECT
            id, deck_id, position, value, suit
        FROM card_to_deck
        WHERE deck_id = $1 AND position = (SELECT (position) FROM decks WHERE id = $1)
        "#,
        deck_id
    )
    .fetch_one(&mut tx)
    .await
    .unwrap();

    sqlx::query!(
        r#"
        UPDATE decks
            SET position = position + 1
        WHERE
            id = $1
    "#,
        deck_id
    )
    .execute(&mut tx)
    .await
    .unwrap();

    tx.commit().await.unwrap();

    Ok(card)
}

pub async fn draw_n(
    db: crate::Db,
    deck_id: DeckId,
    n: i32,
) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let mut tx = db.begin().await.unwrap();

    let cards = sqlx::query_as!(
        Card,
        r#"
        SELECT
            id, deck_id, position, value, suit
        FROM card_to_deck
        WHERE
            deck_id = $1
            AND position >= (SELECT position from decks WHERE id = $1)
            AND position < (SELECT position from decks WHERE id = $1) + $2
        ORDER by position

    "#,
        deck_id,
        n
    )
    .fetch_all(&mut tx)
    .await
    .unwrap();

    sqlx::query!(
        r#"
        UPDATE decks
            SET position = position + $2
        WHERE
            id = $1
    "#,
        deck_id,
        n
    )
    .execute(&mut tx)
    .await
    .unwrap();

    tx.commit().await.unwrap();

    Ok(cards)
}
