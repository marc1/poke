use super::{ Card, Rank, Suit };
use indexmap::IndexSet;

// a wrapper around IndexSet<Card>
pub struct CardSet {
    cards: IndexSet<Card>
}

impl CardSet {
    pub fn new(a: &[Card]) -> Self {
	todo!()
    }
}


