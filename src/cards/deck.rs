use crate::cards::Card;
use crate::cards::Suit;
use crate::cards::Rank;
use crate::cards::Hand;

use rand::prelude::*;

pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn new(inc_jokers: bool) -> Deck {
        let hearts      = create_cards_for(Suit::Hearts);
        let diamonds    = create_cards_for(Suit::Diamonds);
        let spades      = create_cards_for(Suit::Spades);
        let clubs       = create_cards_for(Suit::Clubs);

        let mut cards = Vec::new();
        cards.extend(hearts);
        cards.extend(diamonds);
        cards.extend(spades);
        cards.extend(clubs);

        if inc_jokers == true {
            cards.push(Card::new(Suit::Other, -1, Rank::JokerRed));
            cards.push(Card::new(Suit::Other, -2, Rank::JokerBlack));
        }

        Deck {
            cards
        }
    }

    pub fn shuffle(&mut self) {
        let length = self.cards.len();
        for i in 0..length {
            let i2 = rand::thread_rng().gen_range(0..length);
            self.cards.swap(i, i2);
        }
    }

    pub fn remove_card(&mut self, suit: &Suit, number: i8) -> bool {
        let i = self.cards.iter().position(|card| card.suit == *suit && card.value == number);

        match i {
            Some(index) => {
                self.cards.remove(index);
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    pub fn deal(&mut self, hands: &mut Vec<Hand>, quantity: i8) {
        for _i in 0..quantity {
              for hand in hands.iter_mut() {
                  self.top_card_to_hand(hand);
            }
        }
    }

    pub fn top_card_to_hand(&mut self, hand: &mut Hand) {
        if self.cards.len() > 0 {
            let card = self.cards.pop().unwrap();
            hand.cards.push(card);
        }
    }
}

fn create_cards_for(suit: Suit) -> Vec<Card> {
    let mut result = Vec::new();

    for number in 2..=10 {
        result.push(Card::new(suit, number, Rank::Number));
    }

    result.push(Card::new(suit, 10, Rank::Jack));
    result.push(Card::new(suit, 10, Rank::Queen));
    result.push(Card::new(suit, 10, Rank::King));
    result.push(Card::new(suit, 1, Rank::Ace));

    return result;
}

#[cfg(test)]
pub mod test {
    use crate::cards::{Deck, Rank};
    use crate::cards::Suit;
    use crate::cards::Hand;

    #[test]
    fn deck_deal_test() {
        let mut deck = Deck::new(false);

        let mut hands: Vec<Hand> = Vec::new();
        hands.push(Hand::new(String::from("Player 1")));
        hands.push(Hand::new(String::from("Player 2")));

        deck.deal(&mut hands, 4);

        let hand1_cards = &hands[0].cards;
        let hand2_cards = &hands[1].cards;

        assert_eq!(hand1_cards.len(), 4);
        assert_eq!(hand2_cards.len(), 4);

        deck.top_card_to_hand(&mut hands[0]);
        deck.top_card_to_hand(&mut hands[1]);

        assert_eq!(hands[0].cards.len(), 5usize);
        assert_eq!(hands[1].cards.len(), 5usize);
    }

    #[test]
    fn deck_create_test() {
        let deck = Deck::new(false);
        assert_eq!(deck.cards.len(), 52);

        let card = &deck.cards[0];
        assert_eq!(card.value, 2);
        assert_eq!(card.suit, Suit::Hearts);

        assert_suit(&deck, &Suit::Hearts);
        assert_suit(&deck, &Suit::Diamonds);
        assert_suit(&deck, &Suit::Spades);
        assert_suit(&deck, &Suit::Clubs);
    }

    #[test]
    fn deck_create_jokers_test() {
        let deck = Deck::new(true);
        assert_eq!(deck.cards.len(), 54);
    }

    #[test]
    fn remove_card() {
        let mut deck = Deck::new(true);
        assert_eq!(deck.cards.len(), 54);

        assert_eq!(deck.remove_card(&Suit::Other, -1), true);
        assert_eq!(deck.cards.len(), 53);

        assert_eq!(deck.remove_card(&Suit::Other, -1), false);
        assert_eq!(deck.cards.len(), 53);

        assert_eq!(deck.remove_card(&Suit::Other, -2), true);
        assert_eq!(deck.cards.len(), 52);

        assert_eq!(deck.remove_card(&Suit::Other, -2), false);
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn shuffle_cards() {
        let mut deck = Deck::new(true);
        deck.shuffle();
    }

    fn assert_suit(deck: &Deck, suit: &Suit) {
        for number in 2..=10 {
            assert_eq!(has_card(&deck, &suit, number, Rank::Number), true);
        }

        assert_eq!(has_card(&deck, &suit, 10, Rank::Jack), true);
        assert_eq!(has_card(&deck, &suit, 10, Rank::Queen), true);
        assert_eq!(has_card(&deck, &suit, 10, Rank::King), true);
        assert_eq!(has_card(&deck, &suit, 1, Rank::Ace), true);
    }

    fn has_card(deck: &Deck, suit: &Suit, number: i8, rank: Rank) -> bool {
        let result = deck.cards.iter().find(|&item| item.suit == *suit && item.value == number && item.rank == rank);

        match result {
            None => false,
            _ => true
        }
    }
}