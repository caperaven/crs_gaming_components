use crate::cards::Suit;
use crate::cards::Rank;

pub struct Card {
    pub suit    : Suit,
    pub value   : i8,
    pub rank    : Rank
}

impl Card {
    pub fn new(suit: Suit, value: i8, rank: Rank) -> Card {
        Card {
            suit,
            value,
            rank
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cards::{Card, Suit, Rank};

    #[test]
    fn card_create_test() {
        let card = Card::new(Suit::Clubs, 1, Rank::Number);
        assert_eq!(card.value, 1);
        assert_eq!(card.suit, Suit::Clubs);
        assert_eq!(card.rank, Rank::Number);
    }
}
