use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use rand::thread_rng;

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
pub enum Mode {
    SuitGame(Suit),
    Null,
    Grand,
    Bidding,
}

#[derive(PartialEq, Debug)]
pub enum Actions {
    Bid(u8),
    Pass,
    TakeSkat,
    ReturnSkat,
    Announce(Mode),
    Cards(Vec<Card>),
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
        game_type: Mode::Bidding,
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

pub fn available_actions(round: &Round) -> Vec<Actions> {
    let mut actions = Vec::new();
    match round.state.game_type {
        Mode::Bidding => {
            actions.push(Actions::Bid(get_next_bid(round)));
            actions.push(Actions::Pass);
        }
        _ => (),
    }
    return actions;
}

fn get_next_bid(round: &Round) -> u8 {
    match round.state.bid {
        0 => 18,
        18 => 20,
        20 => 22,
        22 => 23,
        23 => 24,
        24 => 27,
        27 => 30,
        30 => 33,
        33 => 35,
        35 => 36,
        36 => 40,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_round() {
        let round = new_round();
        assert_eq!(
            round.state,
            State {
                bid: 0,
                modifier: 1,
                turn: 0,
                game_type: Mode::Bidding
            }
        );
        assert_eq!(round.forehand.len(), 10);
        assert_eq!(round.middlehand.len(), 10);
        assert_eq!(round.rearhand.len(), 10);
        assert_eq!(round.skat.len(), 2);
        assert_eq!(round.trick.len(), 0);
    }

    #[test]
    fn test_get_avialable_actions() {
        let mut round = Round {
            state: State {
                bid: 0,
                modifier: 1,
                turn: 0,
                game_type: Mode::Bidding,
            },
            forehand: Vec::new(),
            middlehand: Vec::new(),
            rearhand: Vec::new(),
            skat: Vec::new(),
            trick: Vec::new(),
        };

        assert_eq!(
            available_actions(&round),
            vec![Actions::Bid(18), Actions::Pass]
        );

        round.state.bid = 33;
        assert_eq!(
            available_actions(&round),
            vec![Actions::Bid(35), Actions::Pass]
        );
    }
}
