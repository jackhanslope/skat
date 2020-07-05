use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone, Debug)]
pub enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Rank {
    Seven,
    Eight,
    Nine,
    Queen,
    King,
    Ten,
    Ace,
    Jack,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
