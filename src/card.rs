#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn all() -> Vec<Suit> {
        vec![
            Suit::Clubs,
            Suit::Diamonds,
            Suit::Hearts,
            Suit::Spades,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
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

impl Rank {
    pub fn all() -> Vec<Rank> {
        vec![
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }

    pub fn rank(&self) -> &Rank {
        &self.rank
    }

    pub fn compare_bridge_value(&self, other: &Card) -> std::cmp::Ordering {
        // Compare the ranks
        let rank_comparison = self.rank().cmp(other.rank());
        // If the ranks are equal, compare the suits
        if rank_comparison == std::cmp::Ordering::Equal {
            self.suit().cmp(other.suit())
        } else {
            rank_comparison
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ace_of_spades() {
        let card = Card {
            suit: Suit::Spades,
            rank: Rank::Ace,
        };
        assert_eq!(*card.suit(), Suit::Spades);
        assert_eq!(*card.rank(), Rank::Ace);
    }

    #[test]
    fn suit() {
        let card = Card::new(Suit::Hearts, Rank::Two);
        assert_eq!(*card.suit(), Suit::Hearts);
    }

    #[test]
    fn rank() {
        let card = Card::new(Suit::Hearts, Rank::Two);
        assert_eq!(*card.rank(), Rank::Two);
    }

    #[test]
    fn ace_higher_than_king() {
        let ace = Card::new(Suit::Spades, Rank::Ace);
        let king = Card::new(Suit::Spades, Rank::King);

        assert!(ace.rank() > king.rank());
    }

    #[test]
    fn deuce_lower_than_three() {
        let deuce = Card::new(Suit::Spades, Rank::Two);
        let three = Card::new(Suit::Spades, Rank::Three);

        assert!(deuce.rank() < three.rank());
    }

    #[test]
    fn ace_of_spades_equal_to_ace_of_spades() {
        let ace1 = Card::new(Suit::Spades, Rank::Ace);
        let ace2 = Card::new(Suit::Spades, Rank::Ace);

        assert_eq!(ace1, ace2);
    }

    #[test]
    fn same_suits_equals() {
        let card1 = Card::new(Suit::Hearts, Rank::Two);
        let card2 = Card::new(Suit::Hearts, Rank::Three);

        assert_eq!(card1.suit(), card2.suit());
    }

    #[test]
    fn three_of_clubs_less_than_three_of_diamonds() {
        let three_of_clubs = Card::new(Suit::Clubs, Rank::Three);
        let three_of_diamonds = Card::new(Suit::Diamonds, Rank::Three);

        assert_eq!(
            three_of_clubs.compare_bridge_value(&three_of_diamonds),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn compare_bridge_value() {
        let aces = vec![
            Card::new(Suit::Diamonds, Rank::Ace),
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Clubs, Rank::Ace),
        ];
        let mut sorted_aces = aces.clone();
        sorted_aces.sort_by(|a, b| a.compare_bridge_value(b));

        assert_eq!(
            sorted_aces,
            vec![
                Card::new(Suit::Clubs, Rank::Ace),
                Card::new(Suit::Diamonds, Rank::Ace),
                Card::new(Suit::Hearts, Rank::Ace),
                Card::new(Suit::Spades, Rank::Ace),
            ],
        );
    }
}
