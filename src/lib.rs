use std::{convert::{From, TryFrom}};
use snafu::{prelude::*, Whatever};
use roaring::RoaringBitmap;

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Card {
    rank: Rank,
    suit: Suit
}

impl From<Card> for u8 {
    fn from(c: Card) -> u8 {
	let r_idx: u8 = c.rank.into();
	let s_idx: u8 = c.suit.into();

	// 2s, 2h, 2d, 2c. 3s, 3h, 3d, 3c... Ks, Kh, Kd, Kc. As, Ah, Ad, Ac.
	// say we want to find the index of 3c (7). the Rank::Three has an index of 1, and the Suit::Club has an index of 3.

	r_idx*4 + s_idx
    }
}

impl TryFrom<(u8, u8)> for Card {
    type Error = Whatever;

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
	Ok(Card { rank: Rank::try_from(value.0)?, suit: Suit::try_from(value.1)? })
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
	    let r_idx = value-s_idx;

	    Ok((r_idx/4, s_idx).try_into()?)
	}
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn u8_into_card() -> Result<(), Whatever> {
	// We can use this and map each Card to an integer from 0..51.
	let mut map = RoaringBitmap::new();
	map.insert_range(0..52);

	for i in 0..52 {
	    let c: Card = i.try_into()?;
	    println!("{:?}", c);
	}

	Ok(())
    }
}
