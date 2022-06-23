use snafu::{prelude::*, Whatever};
use std::convert::{From, TryFrom};

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Rank {
    Two,
    Three,
    King,
    Ace,
}

impl From<Rank> for u8 {
    fn from(rank: Rank) -> Self {
        rank as Self
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Suit {
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

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
struct Card(u8);

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

#[cfg(test)]
mod tests {

    #[test]
    fn foo() {
        use crate::*;
    }
}
