use crate::{handlers::user::Action, models::user::UserId};
use serde::{Deserialize, Serialize};
use tracing::*;
use uuid::Uuid;
use crate::prelude::*;

use super::{
    card::{Card, CardId},
    deck::{create_deck_transaction, DeckId},
    hand::HandId,
};

pub type GameId = Uuid;
pub type Chips = i32;
pub type SeatNumber = i32;

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub id: GameId,
    pub name: String,
    pub owner_id: UserId,
    pub deck_id: DeckId,
    pub button_seat_number: SeatNumber,
    pub active_seat_number: SeatNumber,
    pub last_to_bet_seat_number: Option<SeatNumber>,
    pub flop_1_card_id: Option<CardId>,
    pub flop_2_card_id: Option<CardId>,
    pub flop_3_card_id: Option<CardId>,
    pub turn_card_id: Option<CardId>,
    pub river_card_id: Option<CardId>,
    pub pot: Chips,
    pub status: String,
    pub phase: String,
}

impl Game {

    pub async fn accept_player_move(db: Db, game_id: GameId, user_id: UserId, action: Action) -> Result<()> {
        let game = Self::fetch(db.clone(), game_id).await?;

        if game.status() != GameStatus::Running {
            return Err("game is not running".into())
        }

        if !Self::player_is_active_player(db.clone(), game_id, user_id).await? {
            return Err("player is not active player".into())
        }

        match action {
            Action::Bet { amount} => Self::bet_player(db.clone(), game_id, user_id, amount).await?,
            Action::Fold  => Self::fold_player(db.clone(), game_id, user_id).await?,
            _ => todo!()
        }

        // if the phase isn't over, advance the game
        if !Self::player_is_last_to_act(db.clone(), game_id, user_id).await? {
            Self::advance_to_next_player(db.clone(), game_id).await?;
            return Ok(())
        }


        // round is over
        match game.phase() {
            GamePhase::PreFlop => Self::deal_flop(db, game_id).await?,
            GamePhase::Flop => Self::deal_turn(db, game_id).await?,
            GamePhase::Turn => Self::deal_river(db, game_id).await?,
            GamePhase::River => Self::end_round(db, game_id).await?,
        }

        Ok(())
    }


    pub async fn start_game(db: Db, game_id: GameId) -> Result<()> {
        sqlx::query!("UPDATE games set status='running' where id=$1", game_id).execute(&db).await?;
        Ok(())
    }

    pub async fn pause_game(db: Db, game_id: GameId) -> Result<()> {
        sqlx::query!("UPDATE games set status='paused' where id=$1", game_id).execute(&db).await?;
        Ok(())
    }


    async fn advance_to_next_player(db: Db, game_id: GameId) -> Result<()> {
        todo!()
    }

    fn phase(&self) -> GamePhase {
        GAME_PHASES.get(self.phase.as_str()).cloned().unwrap()
    }

    fn status(&self) -> GameStatus {
        GAME_STATUSES.get(self.status.as_str()).cloned().unwrap()
    }

    pub async fn list(db: Db) -> Result<Vec<Game>> {
        sqlx::query_as!(Game, "SELECT * FROM games")
            .fetch_all(&db)
            .await
            .map(Ok)
            .unwrap()
    }

    pub async fn fetch(db: Db, id: GameId) -> Result<Game> {
        sqlx::query_as!(Game, "SELECT * FROM games WHERE id = $1", id)
            .fetch_one(&db)
            .await
            .map(Ok)
            .unwrap()
    } 

    pub async fn create(
        db: Db,
        name: String,
        owner_id: UserId,
    ) -> Result<Game> {
        let tx = db.begin().await.unwrap();
        let (mut tx, deck_id) = create_deck_transaction(tx).await.unwrap();
    
        let game = sqlx::query_as!(
            Game,
            "INSERT INTO games (name, owner_id, deck_id, status) VALUES ($1, $2, $3, DEFAULT) RETURNING *",
            name,
            owner_id,
            deck_id
        )
        .fetch_one(&mut tx)
        .await
        .unwrap();
    
        tx.commit().await.unwrap();
    
        Ok(game)
    }

    async fn remove_player_cards<'a>(
        mut tx: Tx<'a>,
        game_id: GameId,
    ) -> Result<Tx<'a>> {
        sqlx::query!(
            r#"
                delete from hands where id in (select hand_id from players where game_id=$1)
            "#,
            game_id
        )
        .execute(&mut tx)
        .await?;
    
        Ok(tx)
    }

    pub async fn bet_player(
        db: Db,
        game_id: GameId,
        user_id: UserId,
        amount: Chips,
    ) -> Result<()> {
        todo!()
    }

    pub async fn stand_player(
        db: Db,
        game_id: GameId,
        user_id: UserId,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            update players
                set status = 'spectating',
                    seat_number = null
                where
                    user_id = $1 and
                    game_id = $2
            "#,
            user_id,
            game_id
        )
        .execute(&db)
        .await
        .unwrap();
    
        Ok(())
    }

    pub async fn sit_player_at_first_available_seat(
        db: Db,
        game_id: GameId,
        user_id: UserId,
    ) -> Result<SeatNumber> {
        let seat = sqlx::query_scalar!(
            r#"
                update players
                    set status = 'playing',
                        seat_number = foo.next_seat_number
                            from (select min(bizbaz.seat_number) as next_seat_number from (
                            select generate_series as seat_number from generate_series(0, 5) except
                            select seat_number from players where
                                game_id = $1 and 
                                seat_number is not null
                        ) as bizbaz) as foo
                        where
                            user_id = $2 and seat_number is null and game_id = $1
                returning (seat_number)
            "#,
            game_id,
            user_id
        )
        .fetch_one(&db)
        .await
        .unwrap()
        .unwrap();
    
        Ok(seat)
    }
    
    pub async fn leave_game(
        db: Db,
        game_id: GameId,
        user_id: UserId,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                DELETE FROM players WHERE user_id = $1 AND game_id = $2
            "#,
            user_id,
            game_id
        )
        .execute(&db)
        .await
        .unwrap();
    
        Ok(())
    }

    pub async fn sit_player_at_seat(
        db: Db,
        game_id: GameId,
        user_id: UserId,
        seat_number: SeatNumber,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            update players
                set seat_number = $1,
                    status = 'playing'
                where
                    user_id =  $2 AND
                    game_id = $3
            "#,
            seat_number,
            user_id,
            game_id,
        )
        .execute(&db)
        .await
        .unwrap();
    
        Ok(())
    }

    pub async fn fold_player(
        db: Db,
        game_id: GameId,
        user_id: UserId,
    ) -> Result<()> {
        todo!()
    }


    async fn deal_flop(db: Db, game_id: GameId) -> Result<()> {
        let mut tx = db.begin().await.unwrap();

        sqlx::query!(
            r#"
                with current_deck as (
                    select * from decks where id = (select deck_id from games where id = $1)
                )
                update games
                    set
                        flop_1_card_id = (select id from card_to_deck where deck_id = games.deck_id AND position = (select position from current_deck) + 1),
                        flop_2_card_id = (select id from card_to_deck where deck_id = games.deck_id AND position = (select position from current_deck) + 2),
                        flop_3_card_id = (select id from card_to_deck where deck_id = games.deck_id AND position = (select position from current_deck) + 3)
                where
                    id = $1
            "#,
            game_id
        ).execute(&mut tx).await.unwrap();

        sqlx::query!(
            r#"
                update decks
                    set
                        position = position + 4
                where id = (select deck_id from games where id = $1)
            "#,
            game_id
        )
        .execute(&mut tx)
        .await
        .unwrap();

        tx.commit().await.unwrap();

        Ok(())
    }

    async fn deal_turn(db: Db, game_id: GameId) -> Result<()> {
        let mut tx = db.begin().await.unwrap();
    
        debug!("IN HERE");
    
        sqlx::query!(
            r#"
                with current_deck as (
                    select * from decks where id = (select deck_id from games where id = $1)
                )
                update games
                    set
                        turn_card_id = (select id from card_to_deck where deck_id = games.deck_id AND position = (select position from current_deck) + 1)
                where
                    id = $1
            "#,
            game_id
        ).execute(&mut tx).await.unwrap();
    
        sqlx::query!(
            r#"
                update decks
                    set
                        position = position + 2
                where id = (select deck_id from games where id = $1)
            "#,
            game_id
        )
        .execute(&mut tx)
        .await
        .unwrap();
    
        tx.commit().await.unwrap();
    
        Ok(())
    }

    pub async fn deal_cards_to_players(
        db: Db,
        game_id: GameId,
    ) -> Result<Vec<(UserId, Vec<Card>)>> {
        // sqlx::query!(
        //     r#"
        //         with current_players as (
        //             select * from players where game_id = $1
        //         )
        //         update players
        //             set
    
        //         where
        //             game_id=$1
        //     "#,
        //     game_id
        // )
        // .execute(&context.db)
        // .await
        // .unwrap();
    
        /*
        0: 0 6
        1: 1 7
        2: 2 8
        3: 3 9
        4: 4 10
        5: 5 11
        */
        todo!()
    }

    pub async fn deal_river(db: Db, game_id: GameId) -> Result<()> {
        let mut tx = db.begin().await.unwrap();

        debug!("IN HERE");

        sqlx::query!(
            r#"
                with current_deck as (
                    select * from decks where id = (select deck_id from games where id = $1)
                )
                update games
                    set
                        river_card_id = (select id from card_to_deck where deck_id = games.deck_id AND position = (select position from current_deck) + 1)
                where
                    id = $1
            "#,
            game_id
        ).execute(&mut tx).await.unwrap();

        sqlx::query!(
            r#"
                update decks
                    set
                        position = position + 2
                where id = (select deck_id from games where id = $1)
            "#,
            game_id
        )
        .execute(&mut tx)
        .await
        .unwrap();

        tx.commit().await.unwrap();
        Ok(())
    }

    pub async fn remove_community_cards<'a>(
        mut tx: Tx<'a>,
        game_id: GameId,
    ) -> Result<Tx<'a>> {
        sqlx::query!(
            r#"
                update games
                set
                    flop_1_card_id = null, 
                    flop_2_card_id = null, 
                    flop_3_card_id = null, 
                    turn_card_id = null, 
                    river_card_id = null
                where
                    id = $1
            "#,
            game_id
        )
        .execute(&mut tx)
        .await?;
        Ok(tx)
    }


    pub async fn shuffle_game_deck_transaction<'a>(
        mut tx: Tx<'a>,
        game_id: GameId,
    ) -> Result<Tx<'a>> {
        let deck_id = sqlx::query_scalar!("select deck_id from games where id = $1", game_id)
            .fetch_one(&mut tx)
            .await?;

        let tx = super::deck::shuffle_deck_transaction(tx, deck_id).await?;

        Ok(tx)
    }

    async fn distribute_winnings<'a>(
        tx: Tx<'a>,
        game_id: GameId,
    ) -> Result<Tx<'a>> {
        todo!()
    }

    async fn round_is_over(
        db: Db,
        game_id: GameId,
    ) -> Result<bool> {
        todo!()
    }

    pub async fn player_is_active_player(
        db: Db,
        game_id: GameId,
        user_id: UserId,
    ) -> Result<bool> {
        let maybe_active_player = sqlx::query_as!(
            GameSessionDto, 
            r#"
                select * from players where game_id = $1 AND user_id = $2 and seat_number = (select active_seat_number from games where id = $1)
            "#,
            game_id,
            user_id
        ).fetch_optional(&db).await?;

        match maybe_active_player {
            Some(player) if player.user_id == user_id => Ok(true),
            _ => Ok(false)
        }
    }

    async fn player_is_last_to_act(
        db: Db,
        game_id: GameId,
        user_id: UserId,
    ) -> Result<bool> {
        todo!()
    }

    async fn advance_button<'a>(
        tx: Tx<'a>,
        game_id: GameId,
    ) -> Result<Tx<'a>> {
        todo!()
    }

    async fn game_can_continue<'a>(
        db: Db,
        game_id: GameId,
    ) -> Result<bool> {
        todo!()
    }

    async fn end_round(db: Db, game_id: GameId) -> Result<()> {
        /*
        1. delete community cards
        2. delete player cards
        3. write to stats table
        4. reshuffle cards
        5. distribute pot to winner
        6. advance button
        */
        let tx = db.begin().await.unwrap();

        let tx = Self::remove_community_cards(tx, game_id).await?;
        let tx = Self::remove_player_cards(tx, game_id).await?;
        let tx = Self::shuffle_game_deck_transaction(tx, game_id).await?;
        let tx = Self::distribute_winnings(tx, game_id).await?;
        let tx = Self::advance_button(tx, game_id).await?;

        tx.commit().await.unwrap();

        if Self::game_can_continue(db, game_id).await? {
            todo!()
        } else {
            todo!()
        }
    }


    pub async fn join_game(
        db: Db,
        game_id: GameId,
        user_id: UserId,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO players (
                    user_id,
                    game_id
                ) VALUES (
                    $1,
                    $2
                ) ON CONFLICT DO NOTHING
            "#,
            user_id,
            game_id
        )
        .execute(&db)
        .await
        .unwrap();

        // TODO(will): handle the case that the game doesnt exist
        // TODO(will): handle the case that the user doesn't exist

        Ok(())
    }


}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct GameSessionDto {
    pub user_id: UserId,
    pub game_id: GameId,
    pub stack: Option<Chips>,
    pub bet: Option<Chips>,
    pub hand_id: Option<HandId>,
    pub seat_number: Option<SeatNumber>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub cards: Vec<Card>,
    pub players: Vec<Option<Player>>,
    pub pot: Chips,
    pub active_seat_number: SeatNumber,
    pub button_seat_number: SeatNumber,
    pub status: GameStatus,
    pub you: Option<Player>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: UserId,
    pub bet: Option<Chips>,
    pub hand: Option<Vec<Card>>,
    pub stack: Chips,
    pub status: PlayerStatus,
}


pub async fn get_table(
    _db: Db,
    _game_id: GameId,
) -> Result<Table> {
    todo!()
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlayerStatus {
    Standing,
    Playing,
    Folded,
    Spectating,
}

#[allow(dead_code)]
pub static PLAYER_STATUSES: phf::Map<&'static str, PlayerStatus> = phf::phf_map! {
    "standing" => PlayerStatus::Standing,
    "playing" => PlayerStatus::Playing,
    "folded" => PlayerStatus::Folded,
    "spectating" => PlayerStatus::Spectating,
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum GameStatus {
    Created,
    Running,
    Ended,
    Paused,
}

pub static GAME_STATUSES: phf::Map<&'static str, GameStatus> = phf::phf_map! {
    "created" => GameStatus::Created,
    "running" => GameStatus::Running,
    "ended" => GameStatus::Ended,
    "paused" => GameStatus::Paused,
};


#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    PreFlop,
    Flop,
    Turn,
    River,
}

pub static GAME_PHASES: phf::Map<&'static str, GamePhase> = phf::phf_map! {
    "preFlop" => GamePhase::PreFlop,
    "flop" => GamePhase::Flop,
    "turn" => GamePhase::Turn,
    "river" => GamePhase::River,
};

impl std::str::FromStr for GamePhase {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        GAME_PHASES.get(s).cloned().map(Ok).unwrap_or(Err("status does not exist".into()))
    }
}
