use crate::game::bid_round::BidRound;
use crate::user::User;
use crate::game::lobby::Lobby;
use crate::game::table::Table;

pub mod lobby;
pub mod table;
pub mod bid_round;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Settings {
    pub to_win: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Team {
    Lajvarna,
    Gottarna,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Player<'a> {
    user: &'a User,
    team: Team,
    ready: bool,
}

impl<'a> Player<'a> {
    fn build(user: &'a User, team: Team) -> Player<'a> {
        Player {
            user,
            team,
            ready: false,
        }
    }

    fn user(&self) -> &User {
        self.user
    }

    fn team(&self) -> Team {
        self.team
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Game<'a> {
    settings: Settings,
    table: Table<'a>,
}

impl Game<'_> {
    pub fn new(settings: Settings, table: Table) -> Game {
        Game { settings, table }
    }

    pub fn start_round(&self) -> BidRound {
        BidRound::new(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::bid_round::BidRound;
    use super::*;

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
    fn new_game() {
        let settings = Settings { to_win: 13 };
        let users = setup_users();
        let mut lobby = setup_lobby();
        users.iter().for_each(|u| lobby.add_user(u));
        let table = Table::new(&lobby);
        let game = Game::new(settings, table);

        assert_eq!(game, Game { settings, table });
    }

    #[test]
    fn build_player() {
        let user = User::new("John Doe");
        let player = Player::build(&user, Team::Lajvarna);

        assert_eq!(player.user().name(), "John Doe");
        assert_eq!(player.team(), Team::Lajvarna);
    }

    #[test]
    fn start_round() {
        let settings = Settings { to_win: 13 };
        let users = setup_users();
        let mut lobby = setup_lobby();
        users.iter().for_each(|u| lobby.add_user(u));
        let table = Table::new(&lobby);
        let game = Game::new(settings, table);

        let bid_round = game.start_round();

        assert_eq!(
            bid_round.bids().len(),
            0,
        );
    }
}
