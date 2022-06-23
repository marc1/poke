use std::{convert::{From, TryFrom}};
use snafu::{prelude::*, Whatever};

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
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
    Queen,
    Jack,
    King,
    Ace
}

impl From<Rank> for u8 {
    fn from(r: Rank) -> u8 {
	r as u8
    }
}

impl TryFrom<u8> for Rank {
    type Error = Whatever;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
	Ok(match value {
	    0 => Rank::Two,
	    1 => Rank::Three,
	    2 => Rank::Four,
	    3 => Rank::Five,
	    4 => Rank::Six,
	    5 => Rank::Seven,
	    6 => Rank::Eight,
	    7 => Rank::Nine,
	    8 => Rank::Ten,
	    9 => Rank::Jack,
	    10 => Rank::Queen,
	    11 => Rank::King,
	    12 => Rank::Ace,
	    _ => whatever!("rank index {} is too large", value)
	})
    }
}

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club
}

impl From<Suit> for u8 {
    fn from(s: Suit) -> u8 {
	s as u8
    }
}

impl TryFrom<u8> for Suit {
    type Error = Whatever;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
	Ok(match value {
	    0 => Suit::Spade,
	    1 => Suit::Heart,
	    2 => Suit::Diamond,
	    3 => Suit::Club,
	    _ => whatever!("suit index {} is too large", value)
	})
    }
}

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Card {
    rank: Rank,
    suit: Suit
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
	Self { rank, suit }
    }
}

impl From<Card> for u8 {
    fn from(c: Card) -> u8 {
	let r_idx: u8 = c.rank.into();
	let s_idx: u8 = c.suit.into();

	r_idx*4 + s_idx
    }
}

impl TryFrom<u8> for Card {
    type Error = Whatever;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
	if value > 51 {
	    whatever!("value {} is too large", value);
	} else {
	    // if idx = rank*4 + suit
	    // suit = idx%4
	    let s_idx = value%4;
	    let r_idx = (value-s_idx)/4;

	    Ok(Card::new(Rank::try_from(r_idx)?, Suit::try_from(s_idx)?))
	}
    }
}
