mod deck;

#[cfg(test)]
mod tests {
    use crate::deck::*;
    use snafu::{prelude::*, Whatever};

    #[test]
    fn card_cmp() {
	let js = Card::new(Rank::Jack, Suit::Spade);
	let ts = Card::new(Rank::Ten, Suit::Spade);

	let jh = Card::new(Rank::Jack, Suit::Heart);

	assert!(js > ts);
	assert!(jh > ts);
	assert!(js > jh);
    }

    #[test]
    fn suit_cmp() {
	assert!(Suit::Spade > Suit::Club)
    }
        
    #[test]
    fn foo() -> Result<(), Whatever> {
	use std::str::FromStr;

	let mut board = CardSet::new();
	board.inner.insert("Js".parse()?);
	board.inner.insert("Th".parse()?);
	board.inner.insert("7d".parse()?);
	board.inner.insert("9s".parse()?);
	board.inner.insert("Tc".parse()?);
	board.inner.insert("Ah".parse()?);
	
	for c in board.inner.iter().rev() {
	    println!("{}", c);
	}

	Ok(())
    }
}
