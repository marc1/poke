mod deck;
    
#[cfg(test)]
mod tests {
    use snafu::{prelude::*, Whatever};

    type Result<T> = std::result::Result<T, Whatever>;

    #[test]
    fn card_set() -> Result<()> {
	use crate::deck::*;
	
	let a = Card::new(Rank::Ace, Suit::Spade);
	let b = Card::new(Rank::Ten, Suit::Spade);
	let c = Card::new(Rank::Two, Suit::Heart);

	//let set = CardSet::new(&[a, b, c]);
	println!("{}", std::mem::size_of::<Card>());

	Ok(())
    }
    
    #[test]
    fn card_to_u8() -> Result<()> {
	use crate::deck::*;
	
	let c = Card::new(Rank::Ace, Suit::Spade);

	assert_eq!(u8::from(c), 48);
	
	Ok(())
    }

    #[test]
    fn u8_to_card() -> Result<()> {
	use crate::deck::*;
	
	let n: u8 = 33;

	assert_eq!(n, Card::new(Rank::Ten, Suit::Heart).into());

	Ok(())
    }
}
