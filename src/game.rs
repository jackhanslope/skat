use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Round {
    state: State,
    forehand: Vec<Card>,
    middlehand: Vec<Card>,
    rearhand: Vec<Card>,
    skat: Vec<Card>,
    trick: Vec<Card>,
}

#[derive(PartialEq, Debug)]
struct State {
    bid: u8,
    modifier: u8,
    turn: u8,
    game_type: Mode,
}

#[derive(PartialEq, Debug)]
enum Mode {
    SuitGame(Suit),
    Null,
    Grand,
    NotStarted,
}

pub fn new_round() -> Round {
    let ranks = [
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Queen,
        Rank::King,
        Rank::Ten,
        Rank::Ace,
        Rank::Jack,
    ];

    let suits = [Suit::Club, Suit::Spade, Suit::Heart, Suit::Diamond];

    let mut deck = Vec::new();

    for s in suits.iter() {
        for r in ranks.iter() {
            deck.push(Card { suit: *s, rank: *r })
        }
    }

    deck.shuffle(&mut thread_rng());

    let state = State { 
        bid: 0,
        modifier: 1,
        turn: 0,
        game_type: Mode::NotStarted 
    };

    return Round {
        state: state,
        forehand: deck[0..10].to_vec(),
        middlehand: deck[10..20].to_vec(),
        rearhand: deck[20..30].to_vec(),
        skat: deck[30..].to_vec(),
        trick: Vec::new(),
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_round() {
        let round = new_round();
        assert_eq!(round.state, State{ bid: 0, modifier: 1, turn: 0, game_type: Mode::NotStarted });
        assert_eq!(round.forehand.len(), 10);
        assert_eq!(round.middlehand.len(), 10);
        assert_eq!(round.rearhand.len(), 10);
        assert_eq!(round.skat.len(), 2);
        assert_eq!(round.trick.len(), 0);
    }
}
