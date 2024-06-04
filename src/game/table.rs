use crate::card::Card;
use crate::game::lobby::Lobby;
use crate::game::Player;
use crate::deck::Deck;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Table<'a> {
    north: Player<'a>,
    east: Player<'a>,
    south: Player<'a>,
    west: Player<'a>,
}

impl<'a> Table<'a> {
    pub fn new(lobby: &Lobby<'a>) -> Table<'a> {
        // Each player draws a card
        let high_card_draws = Self::high_card_for_dealer_button(lobby);
        let highest_card_team = high_card_draws[0].0.team();

        // Highest card becomes the dealer at the north position
        // Her team mate becomes the south position
        let dealer_team: Vec<Player> = high_card_draws.iter()
            .filter(|(p, _)| p.team() == highest_card_team)
            .map(|(p, _)| p.clone())
            .collect();

        // The player in the other team with the highest draw sits at the east position
        // Her team mate sits at the west position
        let starting_team: Vec<Player> = high_card_draws.iter()
            .filter(|(p, _)| p.team() != highest_card_team)
            .map(|(p, _)| p.clone())
            .collect();

        Table {
            north: dealer_team[0],
            east: starting_team[0],
            south: dealer_team[1],
            west: starting_team[1],
        }
    }

    fn high_card_for_dealer_button(lobby: &Lobby<'a>) -> Vec<(Player<'a>, Card)> {
        let mut deck = Deck::new();
        deck.shuffle();

        let mut player_cards: Vec<(Player, Card)> = lobby.players.iter().map(|p| {
            let card = deck.cards.pop().expect("Deck should have enough cards");
            ((*p).clone(), card)
        }).collect();

        player_cards.sort_by(|a, b| {
            a.1.compare_bridge_value(&b.1)
        });

        player_cards
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::lobby::Lobby;
    use crate::game::{Settings};
    use crate::user::User;

    fn setup_users() -> Vec<User> {
        vec![
            User::new("A"),
            User::new("B"),
            User::new("C"),
            User::new("D"),
        ]
    }

    fn setup_lobby() -> Lobby<'static> {
        let settings = Settings { to_win: 13 };
        Lobby::new(settings)
    }

    #[test]
    fn high_card_for_dealer_button() {
        let users = setup_users();
        let mut lobby = setup_lobby();
        users.iter().for_each(|u| lobby.add_user(u));

        let high_card_draws = Table::high_card_for_dealer_button(&lobby);

        let cards = high_card_draws.iter()
            .map(|(_, c)| c.clone())
            .collect::<Vec<Card>>();

        let mut cards_sorted_by_bridge_rank = cards.clone();
        cards_sorted_by_bridge_rank.sort_by(|a, b| a.compare_bridge_value(b));

        assert_eq!(
            cards,
            cards_sorted_by_bridge_rank,
        );
    }

    #[test]
    fn new_table() {
        let users = setup_users();
        let mut lobby = setup_lobby();
        users.iter().for_each(|u| lobby.add_user(u));

        let table = Table::new(&lobby);

        assert_eq!(table.north.team(), table.south.team());
        assert_eq!(table.east.team(), table.west.team());
    }
}
