use std::{
    cmp::{Ordering, PartialOrd},
    convert::{From, TryFrom},
    fmt,
    str
};

use snafu::{prelude::*, Whatever};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{}", match self {
	    Self::Two => '2',
	    Self::Three => '3',
	    Self::Four => '4',
	    Self::Five => '5',
	    Self::Six => '6',
	    Self::Seven => '7',
	    Self::Eight => '8',
	    Self::Nine => '9',
	    Self::Ten => 'T',
	    Self::Jack => 'J',
	    Self::Queen => 'Q',
	    Self::King => 'K',
	    Self::Ace => 'A'
	})
    }
}

impl TryFrom<char> for Rank {
    type Error = Whatever;

    fn try_from(c: char) -> Result<Self, Self::Error> {
	Ok(match c {
	    '2' => Self::Two,
	    '3' => Self::Three,
	    '4' => Self::Four,
	    '5' => Self::Five,
	    '6' => Self::Six,
	    '7' => Self::Seven,
	    '8' => Self::Eight,
	    '9' => Self::Nine,
	    'T' => Self::Ten,
	    'J' => Self::Jack,
	    'Q' => Self::Queen,
	    'K' => Self::King,
	    'A' => Self::Ace,
	    _ => whatever!("invalid rank \"{}\"", c)
	})
    }
}

impl From<Rank> for u8 {
    fn from(rank: Rank) -> Self {
        rank as Self
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{}", match self {
	    Self::Spade => 's',
	    Self::Heart => 'h',
	    Self::Diamond => 'd',
	    Self::Club => 'c'
	})
    }
}

impl TryFrom<char> for Suit {
    type Error = Whatever;
    
    fn try_from(c: char) -> Result<Self, Self::Error> {
	Ok(match c {
	    's' => Self::Spade,
	    'h' => Self::Heart,
	    'd' => Self::Diamond,
	    'c' => Self::Club,
	    _ => whatever!("invalid suit \'{}\'", c)
	})
    }
}

impl From<Suit> for u8 {
    fn from(suit: Suit) -> Self {
        suit as Self
    }
}

impl Ord for Suit {
    fn cmp(&self, other: &Self) -> Ordering {
        // cmp the opposite way since we want Spade to be the largest
        u8::from(*other).cmp(&u8::from(*self))
    }
}

impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{}{}", self.rank(), self.suit())
    }
}

impl str::FromStr for Card {
    type Err = Whatever;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
	let mut it = s.chars();
		
	let rank = Rank::try_from(it.next().unwrap())?;
	let suit = Suit::try_from(it.next().unwrap())?;

	Ok(Card::new(rank, suit))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
	if self.rank() == other.rank() {
	    self.suit().cmp(&other.suit())
	} else {
	    self.rank().cmp(&other.rank())
	}
    }
}
