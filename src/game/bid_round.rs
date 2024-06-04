use crate::deck::Deck;
use crate::hand::Hand;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Bid {
    Pass,
    Play,
}

type WinningBid = Bid;

#[derive(Debug, PartialEq, Clone)]
pub struct BidRound {
    hands: Vec<Hand>,
    dealer: usize,
    bids: Vec<Bid>,
}

impl BidRound {
    pub fn new(dealer: usize) -> BidRound {
        let mut deck = Deck::new();
        deck.shuffle();
        let hands = deck.deal_hands();

        BidRound {
            hands,
            dealer,
            bids: Vec::with_capacity(4),
        }
    }

    pub fn bids(&self) -> &[Bid] {
        &self.bids
    }

    pub fn register_bid(&mut self, bid: Bid) -> Option<WinningBid> {
        self.bids.push(bid);

        if bid == Bid::Play {
            return Some(Bid::Play);
        }

        if self.bids.len() == 4 {
            return Some(Bid::Pass);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_bid_round() {
        let bid_round = BidRound::new(0);

        assert_eq!(bid_round.hands.len(), 4);
        assert_eq!(bid_round.dealer, 0);
        assert_eq!(bid_round.bids().len(), 0);
    }

    #[test]
    fn register_a_play_bid_triggers_play() {
        let mut bid_round = BidRound::new(0);

        assert_eq!(bid_round.register_bid(Bid::Play), Some(Bid::Play));
    }

    #[test]
    fn register_four_pass_bids_triggers_pass() {
        let mut bid_round = BidRound::new(0);

        assert_eq!(bid_round.register_bid(Bid::Pass), None);
        assert_eq!(bid_round.register_bid(Bid::Pass), None);
        assert_eq!(bid_round.register_bid(Bid::Pass), None);
        assert_eq!(bid_round.register_bid(Bid::Pass), Some(Bid::Pass));
    }

    #[test]
    fn register_three_pass_bids_does_not_trigger_pass() {
        let mut bid_round = BidRound::new(0);

        assert_eq!(bid_round.register_bid(Bid::Pass), None);
        assert_eq!(bid_round.register_bid(Bid::Pass), None);
        assert_eq!(bid_round.register_bid(Bid::Pass), None);
    }
}
