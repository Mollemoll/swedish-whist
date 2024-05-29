use crate::user::User;
use crate::game::lobby::Lobby;

mod lobby;
mod table;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Settings {
    to_win: u8,
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
struct Game {
    settings: Settings,
}

impl Game {
    fn new(settings: Settings) -> Lobby<'static> {
        Lobby { settings, players: Vec::new() }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BidRound {
    settings: Settings,
    players: Vec<Player<'static>>,
}

impl BidRound {
    fn new(settings: Settings) -> BidRound {
        // TODO(2024-05-25 mollemoll): move players
        BidRound {  settings, players: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_returns_a_lobby() {
        let settings = Settings { to_win: 13 };
        let game = Game::new(settings);

        assert_eq!(game, Lobby { settings, players: Vec::new() });
    }

    #[test]
    fn build_player() {
        let user = User::new("John Doe");
        let player = Player::build(&user, Team::Lajvarna);

        assert_eq!(player.user().name(), "John Doe");
        assert_eq!(player.team(), Team::Lajvarna);
    }
}
