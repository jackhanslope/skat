use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::{Card, Rank, Suit};

#[derive(Serialize, Deserialize, Debug)]
pub struct Round {
    state: State,
    forehand: [Option<Card>; 10],
    middlehand: [Option<Card>; 10],
    rearhand: [Option<Card>; 10],
    skat: [Option<Card>; 2],
    trick: [Option<Card>; 3],
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct State {
    bids: [u8; 3],
    modifier: u8,
    turn: u8,
    mode: Mode,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Mode {
    SuitGame(Suit),
    Null,
    Grand,
    Bidding,
    Announcing,
    Finished,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Action {
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
        bids: [0; 3],
        modifier: 1,
        turn: 1,
        mode: Mode::Bidding,
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

pub fn available_actions(round: &Round, player: u8) -> Option<Vec<Action>> {
    if player != round.state.turn {
        return None
    }

    let mut actions = Vec::new();
    match round.state.mode {
        Mode::Bidding => {
            let max_bid = round.state.bids.iter().max().unwrap();
            let next_bid = get_next_bid(max_bid);
            let player_bid = round.state.bids[round.state.turn as usize];
            let bid:u8;
            if player_bid == *max_bid 
                || (player_bid == 0 && round.state.turn == 2) // Special case for rear's 1st bid
            { 
                // player must raise
                bid = next_bid;
            } else {
                // player must match
                bid = *max_bid;
            }

            actions.push(Action::Bid(bid));
            actions.push(Action::Pass);
        }
        _ => { return None; },
    }
    return Some(actions);
}

pub fn apply_action(round: &mut Round, action: Action, player: u8) -> bool {
    match available_actions(round, player) {
        None => { return false; }
        Some(aa) => { if !(aa.contains(&action)) { return false } }
    }

    let turn = round.state.turn;
    match action {
        Action::Pass => {
            let bids = &round.state.bids;
            if bids == &[0, 0, 0] {
                match turn {
                    1 => { round.state.turn = 2 }
                    2 => { round.state.turn = 0 }
                    0 => { round.state.mode = Mode::Finished } // everyone passed
                    _ => { panic!() }
                }
            } 
            else if turn == 0 || turn == 1
            {
                // when fore/middle passes rear either won or hasn't bid yet
                round.state.turn = 2;
                if round.state.bids[2] != 0 {
                    // rear has bid therefore rear won
                    round.state.mode = Mode::Announcing;
                }
            } else {
                // when rear passes fore or middle won bidding
                round.state.mode = Mode::Announcing;
                if bids[0] < bids[1] {
                    round.state.turn = 1
                } else {
                    round.state.turn = 0
                }
            }
        }
        Action::Bid(n) => { 
            round.state.bids[turn as usize] = n;
            round.state.turn = get_next_bidder(&round.state.bids);
            return true;
        }
        _ => ()
    }

    return true;
}

fn get_next_bidder(bids: &[u8; 3]) -> u8 {
    match bids {
        // f vs m
        [f, m, 0] if f == m => { return 1 } // f just matched m so m turn
        [f, m, 0] if m > f => { return 0 } // m just raised
        // f vs r
        [f, m, r] if f == r && m <= f => { return 2 }
        [f, m, r] if r > f && m <= f => { return 0 }
        // m vs r
        [f, m, r] if m == r && m > f => { return 2 }
        [f, m, r] if r > m && m > f => { return 1 }

        _ => { panic!() }
    }
}

fn get_next_bid(bid: &u8) -> u8 {
    match bid {
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
        // TODO
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
                bids: [0; 3],
                modifier: 1,
                turn: 1,
                mode: Mode::Bidding
            }
        );
        assert_eq!(round.forehand.len(), 10);
        assert_eq!(round.middlehand.len(), 10);
        assert_eq!(round.rearhand.len(), 10);
        assert_eq!(round.skat.len(), 2);
        assert_eq!(round.trick, [None; 3]);
    }

    #[test]
    fn test_middlehand_win_bidding() {
        let mut round = Round {
            state: State {
                bids: [0; 3],
                modifier: 1,
                turn: 1,
                mode: Mode::Bidding,
            },
            forehand: [None; 10],
            middlehand: [None; 10],
            rearhand: [None; 10],
            skat: [None, None],
            trick: [None; 3],
        };

        assert_eq!(available_actions(&round, 2), None);
        assert_eq!(available_actions(&round, 0), None);

        assert_eq!(available_actions(&round, 1), Some(vec![Action::Bid(18), Action::Pass]));
        assert_eq!(apply_action(&mut round, Action::Bid(18), 1), true);
        assert_eq!((round.state.bids, round.state.turn), ([0, 18, 0], 0));

        assert_eq!(available_actions(&round, 0), Some(vec![Action::Bid(18), Action::Pass]));
        assert_eq!(apply_action(&mut round, Action::Bid(18), 0), true);
        assert_eq!((round.state.bids, round.state.turn), ([18, 18, 0], 1));

        assert_eq!(available_actions(&round, 1), Some(vec![Action::Bid(20), Action::Pass]));
        assert_eq!(apply_action(&mut round, Action::Bid(20), 1), true);
        assert_eq!((round.state.bids, round.state.turn), ([18, 20, 0], 0));

        assert_eq!(available_actions(&round, 0), Some(vec![Action::Bid(20), Action::Pass]));
        assert_eq!(apply_action(&mut round, Action::Pass, 0), true);
        assert_eq!((round.state.bids, round.state.turn), ([18, 20, 0], 2));

        assert_eq!(available_actions(&round, 2), Some(vec![Action::Bid(22), Action::Pass]));
        assert_eq!(apply_action(&mut round, Action::Bid(22), 2), true);
        assert_eq!((round.state.bids, round.state.turn), ([18, 20, 22], 1));

        assert_eq!(available_actions(&round, 1), Some(vec![Action::Bid(22), Action::Pass]));
        assert_eq!(apply_action(&mut round, Action::Bid(22), 1), true);
        assert_eq!((round.state.bids, round.state.turn), ([18, 22, 22], 2));

        assert_eq!(apply_action(&mut round, Action::Pass, 2), true);
        assert_eq!((round.state.bids, round.state.turn), ([18, 22, 22], 1));
        assert_eq!(round.state.mode, Mode::Announcing);
    }

    #[test]
    fn test_forehand_win_bidding() {
        let mut round = Round {
            state: State {
                bids: [0; 3],
                modifier: 1,
                turn: 1,
                mode: Mode::Bidding,
            },
            forehand: [None; 10],
            middlehand: [None; 10],
            rearhand: [None; 10],
            skat: [None, None],
            trick: [None; 3],
        };

        assert_eq!(apply_action(&mut round, Action::Bid(18), 1), true);
        assert_eq!((round.state.bids, round.state.turn), ([0, 18, 0], 0));

        assert_eq!(apply_action(&mut round, Action::Bid(18), 0), true);
        assert_eq!((round.state.bids, round.state.turn), ([18, 18, 0], 1));

        assert_eq!(apply_action(&mut round, Action::Pass, 1), true);
        assert_eq!((round.state.bids, round.state.turn), ([18, 18, 0], 2));

        assert_eq!(available_actions(&round, 2), Some(vec![Action::Bid(20), Action::Pass]));
        assert_eq!(apply_action(&mut round, Action::Pass, 2), true);
        assert_eq!((round.state.bids, round.state.turn), ([18, 18, 0], 0));
        assert_eq!(round.state.mode, Mode::Announcing);
    }

    #[test]
    fn test_rearhand_win_bidding() {
        let mut round = Round {
            state: State {
                bids: [0; 3],
                modifier: 1,
                turn: 1,
                mode: Mode::Bidding,
            },
            forehand: [None; 10],
            middlehand: [None; 10],
            rearhand: [None; 10],
            skat: [None, None],
            trick: [None; 3],
        };

        assert_eq!(apply_action(&mut round, Action::Bid(18), 1), true);
        assert_eq!((round.state.bids, round.state.turn), ([0, 18, 0], 0));

        assert_eq!(apply_action(&mut round, Action::Pass, 0), true);
        assert_eq!((round.state.bids, round.state.turn), ([0, 18, 0], 2));

        assert_eq!(apply_action(&mut round, Action::Bid(20), 2), true);
        assert_eq!((round.state.bids, round.state.turn), ([0, 18, 20], 1));

        assert_eq!(apply_action(&mut round, Action::Pass, 1), true);
        assert_eq!((round.state.bids, round.state.turn), ([0, 18, 20], 2));
        assert_eq!(round.state.mode, Mode::Announcing);
    }

    #[test]
    fn test_all_pass() {
        let mut round = Round {
            state: State {
                bids: [0; 3],
                modifier: 1,
                turn: 1,
                mode: Mode::Bidding,
            },
            forehand: [None; 10],
            middlehand: [None; 10],
            rearhand: [None; 10],
            skat: [None, None],
            trick: [None; 3],
        };

        assert_eq!(apply_action(&mut round, Action::Pass, 1), true);
        assert_eq!((round.state.bids, round.state.turn), ([0, 0, 0], 2));

        assert_eq!(apply_action(&mut round, Action::Pass, 2), true);
        assert_eq!((round.state.bids, round.state.turn), ([0, 0, 0], 0));

        assert_eq!(available_actions(&round, 0), Some(vec![Action::Bid(18), Action::Pass]));
        assert_eq!(apply_action(&mut round, Action::Pass, 0), true);
        assert_eq!((round.state.bids, round.state.turn), ([0, 0, 0], 0));
        assert_eq!(round.state.mode, Mode::Finished);
    }
}
