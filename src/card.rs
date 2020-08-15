use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
