use crate::card::{Card, Suit};
use crate::deck;
use crate::deck::Deck;
use serde::{Deserialize, Serialize};

struct Game {
    player1: Player,
    player2: Player,
    player3: Player,
    deck: Deck,
}

impl Game {
    fn new() -> Game {
        let player1 = Player::new();
        let player2 = Player::new();
        let player3 = Player::new();
        let deck = Deck::new();

        Game {
            player1: player1,
            player2: player2,
            player3: player3,
            deck: deck,
        }
    }
}

// would probably be called a hand in real life, but might want to use the word hand for the set of
// cards a player holds
struct Round {
    state: State,
    mode: Mode,
    deck: Deck,
    forehand: Player,
    middlehand: Player,
    rearhand: Player,
    skat: [Card; 2],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub hand: Vec<Card>,
}

impl Player {
    // init a player with an empty hand
    pub fn new() -> Player {
        let hand: Vec<Card> = Vec::new();

        Player { hand: hand }
    }
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
