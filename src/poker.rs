pub type UserId = u32;
pub type SeatId = u8;
pub type Chips = u32;
pub type MaybeHand = Option<[Card; 2]>;

#[derive(Debug, Clone, Default)]
pub struct Table {
    seats: [Option<User>; 6],
    active_seat: usize,
    pot: Chips,
}

impl Table {
    /// Add a user to the first available seat at the table, returning the id of the seat the user was seated at
    pub fn add_user_to_first_available(
        &mut self,
        user: User,
    ) -> Result<SeatId, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn add_user_to_seat(
        &mut self,
        user: User,
        seat_number: UserId,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn get_user(&self, seat: usize) -> &User {
        todo!()
    }

    pub fn submit_command(&mut self, command: Command) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

#[derive(Debug, Clone, Default)]
pub struct User {
    id: UserId,
    stack: Chips,
    // user has a hand or doesn't
    cards: MaybeHand,
}

#[derive(Debug, Clone)]
pub struct Card {
    value: Value,
    suit: Suit,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

#[derive(Debug, Clone)]
pub struct Command {
    // id of the user command is issued by
    owner_id: UserId,
    kind: CommandKind,
}

#[derive(Debug, Clone)]
pub enum CommandKind {
    Bet { amount: Chips },
    Fold,
}
