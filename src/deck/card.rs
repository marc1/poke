use std::{
    cmp::{Ordering, PartialOrd},
    convert::{From, TryFrom},
};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<Rank> for u8 {
    fn from(rank: Rank) -> Self {
        rank as Self
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl From<Suit> for u8 {
    fn from(suit: Suit) -> Self {
        suit as Self
    }
}

impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // cmp the opposite way since we want Spade to be the largest
        u8::from(*other).partial_cmp(&u8::from(*self))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord)]
#[repr(transparent)]
pub struct Card(u8);

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self(u8::from(rank) | (u8::from(suit) << 4))
    }

    pub fn rank(self) -> Rank {
        unsafe { std::mem::transmute(self.0 & 0b1111) }
    }

    pub fn suit(self) -> Suit {
        unsafe { std::mem::transmute(self.0 >> 4) }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rank() == other.rank() {
            self.suit().partial_cmp(&other.suit())
        } else {
            self.rank().partial_cmp(&other.rank())
        }
    }
}
