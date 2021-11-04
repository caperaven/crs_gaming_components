use crate::cards::Card;

pub struct Hand {
    pub player_id   : String,
    pub cards       : Vec<Card>
}

impl Hand {
    pub fn new(player_id: String) -> Hand {
        Hand {
            player_id,
            cards: Vec::new()
        }
    }

    pub fn sum(&self) -> i8 {
        let mut result = 0;
        for card in self.cards.iter() {
            result += card.value;
        }
        return result;
    }
}

#[cfg(test)]
mod test {
    use crate::cards::{Card, Hand, Suit, Rank};


    #[test]
    fn hand_create_test() {
        let hand = Hand::new("player 1".into());
        assert_eq!(hand.player_id, "player 1");
    }

    #[test]
    fn hand_sum_test() {
        let mut hand = Hand::new("player 1".into());
        hand.cards.push(Card::new(Suit::Clubs, 2, Rank::Number));
        hand.cards.push(Card::new(Suit::Clubs, 3, Rank::Number));
        hand.cards.push(Card::new(Suit::Clubs, 4, Rank::Number));
        hand.cards.push(Card::new(Suit::Clubs, 5, Rank::Number));
        assert_eq!(hand.sum(), 14);
    }
}