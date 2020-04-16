#![allow(unused_variables)]
#![allow(dead_code)]

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {}
#[derive(Copy, Clone, Debug)]
pub enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
}

struct Player {
    hand: Vec<Card>,
}

enum NullModifier {
    Standard,
    Hand,
    Overt,
    HandOvert,
}

enum Mode {
    SuitGame(Suit),
    Null(NullModifier),
    Grand,
}

struct Game {
    state: State,
    mode: Mode,
    deck: Vec<Card>,
    forehand: Player,
    middlehand: Player,
    rearhand: Player,
    skat: [Card; 2],
}

enum State {
    NotStarted,
    Bidding(usize), // u8 to keep track of who's go it is
    Trick(usize),   // u8 to keep track of who's go it is
    Completed,
}

fn new_deck() -> Vec<Card> {
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
