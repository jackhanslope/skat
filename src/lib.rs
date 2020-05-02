// TODO: delete the two lines below before deployment
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub mod card;
pub mod game;

use card::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

pub fn new_deck() -> Vec<Card> {
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

    for s in suits.iter().cloned() {
        for r in ranks.iter().cloned() {
            deck.push(Card { suit: s, rank: r })
        }
    }

    shuffle_deck(deck)
}

fn shuffle_deck(mut deck: Vec<Card>) -> Vec<Card> {
    deck.shuffle(&mut thread_rng());
    deck
}
