#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {
    enum Suit {
        Club,
        Spade,
        Heart,
        Diamond,
    }

    enum Rank {
        Seven,
        Eight,
        Nine,
        Ten,
        Ace,
        Queen,
        Kind,
        Jack,
    }

    struct Card {
        suit: Suit,
        rank: Rank,
    }
}
