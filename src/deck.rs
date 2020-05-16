use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::vec::Vec;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    // returns an unshuffled deck
    pub fn new() -> Deck {
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

        Deck { cards: deck }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}
