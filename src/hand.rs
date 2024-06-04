use crate::card::Card;

#[derive(Debug, PartialEq, Clone)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: Vec::with_capacity(13),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Rank, Suit};
    use super::*;

    #[test]
    fn new_hand_has_no_cards() {
        let hand = Hand::new();

        assert_eq!(hand.cards.len(), 0);
    }

    #[test]
    fn add_card_to_hand() {
        let mut hand = Hand::new();
        hand.add_card(
            Card::new(
                Suit::Spades,
                Rank::Ace,
            )
        );

        assert_eq!(hand.cards.len(), 1);
    }

    #[test]
    fn hand_cards() {
        let mut hand = Hand::new();
        hand.add_card(
            Card::new(
                Suit::Spades,
                Rank::Ace,
            )
        );

        assert_eq!(hand.cards()[0], Card::new(Suit::Spades, Rank::Ace));
    }

    #[test]
    fn hand_len() {
        let mut hand = Hand::new();
        hand.add_card(
            Card::new(
                Suit::Spades,
                Rank::Ace,
            )
        );

        assert_eq!(hand.len(), 1);
    }
}
