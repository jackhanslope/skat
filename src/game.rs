use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Round {
    state: State,
    forehand: [Option<Card>; 10],
    middlehand: [Option<Card>; 10],
    rearhand: [Option<Card>; 10],
    skat: [Option<Card>; 2],
    trick: [Option<Card>; 3],
}

#[derive(PartialEq, Debug)]
struct State {
    bid: [u8; 3],
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
            deck.push(Some(Card { suit: *s, rank: *r }))
        }
    }

    deck.shuffle(&mut thread_rng());

    let state = State {
        bid: [0; 3],
        modifier: 1,
        turn: 0,
        game_type: Mode::Bidding,
    };

    let mut round = Round {
        state: state,
        forehand: [None; 10],
        middlehand: [None; 10],
        rearhand: [None; 10],
        skat: [None; 2],
        trick: [None; 3],
    };

    let mut iter = deck.chunks_exact(10);

    round.forehand.copy_from_slice(iter.next().unwrap());
    round.middlehand.copy_from_slice(iter.next().unwrap());
    round.rearhand.copy_from_slice(iter.next().unwrap());
    round.skat.copy_from_slice(iter.remainder());

    return round;
}

pub fn available_actions(round: &Round, player: u8) -> Option<Vec<Actions>> {
    if player != round.state.turn {
        return None
    }

    let mut actions = Vec::new();
    match round.state.game_type {
        Mode::Bidding => {
            actions.push(Actions::Bid(get_next_bid(round)));
            actions.push(Actions::Pass);
        }
        _ => (),
    }
    return Some(actions);
}

fn get_next_bid(round: &Round) -> u8 {
    match round.state.bid[round.state.turn as usize] {
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
                bid: [0; 3],
                modifier: 1,
                turn: 0,
                game_type: Mode::Bidding
            }
        );
        assert_eq!(round.forehand.len(), 10);
        assert_eq!(round.middlehand.len(), 10);
        assert_eq!(round.rearhand.len(), 10);
        assert_eq!(round.skat.len(), 2);
        assert_eq!(round.trick, [None; 3]);
    }

    #[test]
    fn test_get_avialable_actions() {
        let mut round = Round {
            state: State {
                bid: [0; 3],
                modifier: 1,
                turn: 0,
                game_type: Mode::Bidding,
            },
            forehand: [None; 10],
            middlehand: [None; 10],
            rearhand: [None; 10],
            skat: [None, None],
            trick: [None; 3],
        };

        assert_eq!(
            available_actions(&round, 0),
            Some(vec![Actions::Bid(18), Actions::Pass])
        );

        round.state.bid = [33, 18, 18];
        assert_eq!(
            available_actions(&round, 0),
            Some(vec![Actions::Bid(35), Actions::Pass])
        );
    }
}
