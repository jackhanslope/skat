use crate::card::{Card, Suit};
use crate::deck;
use crate::deck::Deck;
use serde::{Deserialize, Serialize};

struct Game {
    player1: GamePlayer,
    player2: GamePlayer,
    player3: GamePlayer,
    deck: Deck,
    round: Round,
}

impl Game {
    fn new() -> Game {
        let player1 = GamePlayer::new();
        let player2 = GamePlayer::new();
        let player3 = GamePlayer::new();
        let deck = Deck::new();
        let round = Round::new();

        Game {
            player1: player1,
            player2: player2,
            player3: player3,
            deck: deck,
            round: round,
        }
    }
}

// would probably be called a hand in real life, but might want to use the word hand for the set of
// cards a player holds
struct Round {
    state: State,
    mode: Mode,
    forehand: RoundPlayer,
    middlehand: RoundPlayer,
    rearhand: RoundPlayer,
    skat: [Card; 2],
}

impl Round {
    fn new() -> Round {
        let mut deck = Deck::new();
        deck.shuffle();

        let forehand = RoundPlayer::new();
        let middlehand = RoundPlayer::new();
        let rearhand = RoundPlayer::new();

        let skat: [Card; 2] = [deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];

        Round {
            state: State::NotStarted,
            mode: Mode::Bidding,
            forehand: forehand,
            middlehand: middlehand,
            rearhand: rearhand,
            skat: skat,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GamePlayer {
    score: isize,
}

impl GamePlayer {
    // init a player with an empty hand
    pub fn new() -> GamePlayer {
        GamePlayer { score: 0 }
    }
}

pub struct RoundPlayer {
    pub hand: Vec<Card>,
}

impl RoundPlayer {
    pub fn new() -> RoundPlayer {
        let hand: Vec<Card> = Vec::new();

        RoundPlayer { hand: hand }
    }
}

enum Mode {
    Bidding,
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
