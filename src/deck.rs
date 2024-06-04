use crate::card::{
    Card,
    Suit,
    Rank,
};
use crate::hand::Hand;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();

        for suit in Suit::all() {
            for rank in Rank::all() {
                cards.push(Card::new(suit, rank));
            }
        }

        Deck { cards }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        self.cards.shuffle(&mut thread_rng());
    }

    pub fn deal_hands(&mut self) -> Vec<Hand> {
        let mut hands= vec![Hand::new(); 4];

        for _ in 0..13 {
            for hand in hands.iter_mut() {
                hand.add_card(self.cards.pop().expect("Deck should have enough cards"));
            }
        }

        hands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_deck_has_52_cards() {
        let deck = Deck::new();
        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn shuffled_deck_differs_from_new_deck() {
        let mut deck = Deck::new();
        let original = deck.cards.clone();

        deck.shuffle();

        assert_ne!(deck.cards, original);
    }

    #[test]
    fn shuffled_decks_are_different() {
        let mut deck1 = Deck::new();
        let mut deck2 = Deck::new();

        deck1.shuffle();
        deck2.shuffle();

        assert_ne!(deck1.cards, deck2.cards);
    }

    #[test]
    fn deal_hands() {
        let mut deck = Deck::new();
        deck.shuffle();

        let hands = deck.deal_hands();
        assert_eq!(deck.len(), 0);
        assert_eq!(hands.len(), 4);
    }
}
