use crate::user::User;

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
    fn new(settings: Settings) -> GameLobby<'static> {
        GameLobby { settings, players: Vec::new() }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameError {
    RequiresFourReadyPlayers,
    UnbalancedTeams,
}

#[derive(Debug, PartialEq)]
struct GameLobby<'a> {
    settings: Settings,
    players: Vec<Player<'a>>,
}

impl<'a> GameLobby<'a> {
    fn add_user(&mut self, user: &'a User) {
        // Create player
        let player = Player::build(
            &user,
            self.team_to_assign_to()
        );

        // Deny player if already in the game
        if self.players.iter()
            .any(|p| &p.user() == &user) {
            return;
        }

        // Deny player if there are already 4 players
        if self.players.len() == 4 {
            return;
        }

        self.players.push(player);
    }

    fn del_user(&mut self, user: &'a User) {
        self.players.retain(|p| &p.user() != &user);
    }

    fn change_team(&mut self, user: &'a User, team: Team) {
        if let Some(player) = self.players
            .iter_mut()
            .find(|p| p.user() == user) {
                player.team = team;
            }
    }

    fn ready_up(&mut self, user: &'a User) {
        if let Some(player) = self.players
            .iter_mut()
            .find(|p| p.user() == user) {
                player.ready = true;
            }
    }

    fn unready(&mut self, user: &'a User) {
        if let Some(player) = self.players
            .iter_mut()
            .find(|p| p.user() == user) {
                player.ready = false;
            }
    }

    fn ready_count(&self) -> usize {
        self.players.iter()
            .filter(|p| p.ready == true)
            .count()
    }

    fn balanced_teams(&self) -> bool {
        let team_count = self.players.iter()
            .fold((0, 0), |(lajvarna, gottarna), p| {
                match p.team {
                    Team::Lajvarna => (lajvarna + 1, gottarna),
                    Team::Gottarna => (lajvarna, gottarna + 1),
                }
            });

        team_count.0 == 2 && team_count.1 == 2
    }

    fn team_to_assign_to(&mut self) -> Team {
        let count_lajvarna = self.players.iter().filter(|p| p.team == Team::Lajvarna).count();
        let count_gottarna = self.players.iter().filter(|p| p.team == Team::Gottarna).count();

        if count_lajvarna <= count_gottarna {
            Team::Lajvarna
        } else {
            Team::Gottarna
        }
    }

    fn start_game(&self) -> Result<BidRound, GameError> {
        if self.ready_count() != 4 {
            return Err(GameError::RequiresFourReadyPlayers);
        }

        if !self.balanced_teams() {
            return Err(GameError::UnbalancedTeams);
        }

        Ok(BidRound::new(self.settings))
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

    fn setup_game() -> GameLobby<'static> {
        let settings = Settings { to_win: 13 };
        Game::new(settings)
    }

    #[test]
    fn new_game_return_game_lobby() {
        let settings = Settings { to_win: 13 };
        let game_lobby = Game::new(settings);

        assert_eq!(
            game_lobby,
            GameLobby { settings, players: Vec::new() }
        );
    }

    #[test]
    fn add_user_to_lobby() {
        let mut game_lobby = setup_game();
        let user = User::new("John Doe");

        game_lobby.add_user(&user);

        assert_eq!(game_lobby.players.len(), 1);
    }

    #[test]
    fn game_lobby_denies_player_joining_twice() {
        let mut game_lobby = setup_game();
        let user = User::new("John Doe");

        game_lobby.add_user(&user);
        game_lobby.add_user(&user);

        assert_eq!(game_lobby.players.len(), 1);
    }

    #[test]
    fn game_accepts_user_leaving() {
        let mut game_lobby = setup_game();
        let user = User::new("John Doe");
        let user2 = User::new("Jane Doe");

        game_lobby.add_user(&user);
        game_lobby.add_user(&user2);
        game_lobby.del_user(&user);

        assert_eq!(game_lobby.players.len(), 1);
    }


    #[test]
    fn game_lobby_denies_user_leaving_twice() {
        let mut game_lobby = setup_game();
        let user = User::new("John Doe");
        let user2 = User::new("Jane Doe");

        game_lobby.add_user(&user);
        game_lobby.add_user(&user2);
        game_lobby.del_user(&user);
        game_lobby.del_user(&user);

        assert_eq!(game_lobby.players.len(), 1);
    }

    #[test]
    fn users_gets_assigned_to_teams() {
        let mut game_lobby = setup_game();
        let user = User::new("John Doe");
        let user2 = User::new("Jane Doe");
        let user3 = User::new("Dolly");
        let user4 = User::new("Phil");

        game_lobby.add_user(&user);
        game_lobby.add_user(&user2);
        game_lobby.add_user(&user3);
        game_lobby.add_user(&user4);

        assert_eq!(game_lobby.balanced_teams(), true);
    }

    #[test]
    fn game_lobby_denies_5th_player() {
        let mut game_lobby = setup_game();
        let user = User::new("John Doe");
        let user2 = User::new("Jane Doe");
        let user3 = User::new("Dolly");
        let user4 = User::new("Phil");
        let user5 = User::new("John Johnson");

        game_lobby.add_user(&user);
        game_lobby.add_user(&user2);
        game_lobby.add_user(&user3);
        game_lobby.add_user(&user4);
        game_lobby.add_user(&user5);

        assert_eq!(game_lobby.players.len(), 4);
        assert_eq!(game_lobby.players.iter().any(|p| p.user == &user5), false);
    }


    #[test]
    fn player_can_change_team() {
        let mut game_lobby = setup_game();
        let user = User::new("John Doe");

        game_lobby.add_user(&user);
        game_lobby.change_team(&user, Team::Gottarna);

        assert_eq!(
            game_lobby.players.iter()
                .any(|p| p.user == &user && p.team == Team::Gottarna),
            true
        );
    }

    #[test]
    fn player_can_ready_up() {
        let mut game_lobby = setup_game();
        let user = User::new("John Doe");

        game_lobby.add_user(&user);
        game_lobby.ready_up(&user);

        assert_eq!(game_lobby.ready_count(), 1);
    }

    #[test]
    fn readying_up_twice_doesnt_change_anything() {
        let mut game_lobby = setup_game();
        let user = User::new("John Doe");

        game_lobby.add_user(&user);
        game_lobby.ready_up(&user);
        game_lobby.ready_up(&user);

        assert_eq!(game_lobby.ready_count(), 1);
    }

    #[test]
    fn player_can_unready() {
        let mut game_lobby = setup_game();
        let user = User::new("John Doe");
        let user2 = User::new("Jane Doe");

        game_lobby.add_user(&user);
        game_lobby.add_user(&user2);
        game_lobby.ready_up(&user);
        game_lobby.ready_up(&user2);
        game_lobby.unready(&user);

        assert_eq!(game_lobby.ready_count(), 1);
    }

    #[test]
    fn starting_requires_four_ready_players() {
        let mut game_lobby = setup_game();
        let user = User::new("John Doe");
        let user2 = User::new("Jane Doe");
        let user3 = User::new("Dolly");

        game_lobby.add_user(&user);
        game_lobby.add_user(&user2);
        game_lobby.add_user(&user3);
        game_lobby.ready_up(&user);
        game_lobby.ready_up(&user2);
        game_lobby.ready_up(&user3);

        assert_eq!(game_lobby.start_game(), Err(GameError::RequiresFourReadyPlayers));
    }

    #[test]
    fn game_cant_start_with_unbalanced_teams() {
        let mut game_lobby = setup_game();
        let user = User::new("John Doe");
        let user2 = User::new("Jane Doe");
        let user3 = User::new("Dolly");
        let user4 = User::new("Homer");

        game_lobby.add_user(&user);
        game_lobby.add_user(&user2);
        game_lobby.add_user(&user3);
        game_lobby.add_user(&user4);
        game_lobby.change_team(&user4, Team::Lajvarna);
        game_lobby.ready_up(&user);
        game_lobby.ready_up(&user2);
        game_lobby.ready_up(&user3);
        game_lobby.ready_up(&user4);

        assert_eq!(game_lobby.start_game(), Err(GameError::UnbalancedTeams));
    }

    #[test]
    fn game_can_start_with_four_ready_players() {
        let mut game_lobby = setup_game();
        let user = User::new("John Doe");
        let user2 = User::new("Jane Doe");
        let user3 = User::new("Dolly");
        let user4 = User::new("Homer");

        game_lobby.add_user(&user);
        game_lobby.add_user(&user2);
        game_lobby.add_user(&user3);
        game_lobby.add_user(&user4);
        game_lobby.ready_up(&user);
        game_lobby.ready_up(&user2);
        game_lobby.ready_up(&user3);
        game_lobby.ready_up(&user4);

        assert_eq!(
            game_lobby.start_game().unwrap(),
            BidRound {
                settings: Settings { to_win: 13 },
                // TODO(2024-05-25 mollemoll): fix move players
                players: vec![]
            }
        );
    }
}