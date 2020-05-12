use crate::card::{Card, Rank, Suit};
use serde::{Deserialize, Serialize};

struct Game {
    state: State,
    mode: Mode,
    deck: Vec<Card>,
    forehand: Player,
    middlehand: Player,
    rearhand: Player,
    skat: [Card; 2],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub hand: Vec<Card>,
}

enum Mode {
    SuitGame(Suit),
    Null(NullModifier),
    Grand,
}

enum NullModifier {
    Standard,
    Hand,
    Overt,
    HandOvert,
}

enum State {
    NotStarted,
    Bidding(usize), // u8 to keep track of who's go it is
    Trick(usize),   // u8 to keep track of who's go it is
    Completed,
}
