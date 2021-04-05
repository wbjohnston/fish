pub type DeckId = uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Deck {
    pub id: DeckId,
    pub position: i32,
}
use crate::models::card::Card;
use crate::models::card::{SUITS, VALUES};
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

pub async fn shuffle_deck_transaction<'a>(
    mut tx: sqlx::Transaction<'a, sqlx::Postgres>,
    deck_id: DeckId,
) -> Result<sqlx::Transaction<'a, sqlx::Postgres>, Box<dyn std::error::Error>> {
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

pub async fn shuffle_deck<'a>(
    db: crate::Db,
    deck_id: DeckId,
) -> Result<(), Box<dyn std::error::Error>> {
    let tx = db.begin().await.unwrap();
    let tx = shuffle_deck_transaction(tx, deck_id).await?;
    tx.commit().await.unwrap();

    Ok(())
}

pub async fn deal_flop(
    db: crate::Db,
    deck_id: DeckId,
) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    // draw an extra card to burn it
    let cards = draw_n(db, deck_id, 4).await?;

    Ok(cards)
}

pub async fn deal_turn(db: crate::Db, deck_id: DeckId) -> Result<Card, Box<dyn std::error::Error>> {
    // draw an extra card to burn it
    let card = draw_n(db, deck_id, 2).await?;

    Ok(card[0].clone())
}

pub async fn deal_river(
    db: crate::Db,
    deck_id: DeckId,
) -> Result<Card, Box<dyn std::error::Error>> {
    deal_turn(db, deck_id).await
}

pub async fn create_deck(db: crate::Db) -> Result<DeckId, Box<dyn std::error::Error>> {
    let tx = db.begin().await.unwrap();

    let (tx, deck_id) = create_deck_transaction(tx).await?;

    tx.commit().await.unwrap();

    Ok(deck_id)
}

pub async fn create_deck_transaction<'a>(
    mut tx: sqlx::Transaction<'a, sqlx::Postgres>,
) -> Result<(sqlx::Transaction<'a, sqlx::Postgres>, DeckId), Box<dyn std::error::Error>> {
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

    Ok((tx, deck.id))
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
