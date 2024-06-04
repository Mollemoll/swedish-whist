use crate::errors::GameError;
use crate::game::{Game, Player, Settings, Team};
use crate::user::User;
use crate::game::table::Table;

#[derive(Debug, PartialEq, Clone)]
pub struct Lobby<'a> {
    pub(crate) settings: Settings,
    pub(crate) players: Vec<Player<'a>>,
}

impl<'a> Lobby<'a> {
    pub fn new(settings: Settings) -> Lobby<'a> {
        Lobby {
            settings,
            players: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: &'a User) {
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

    pub fn ready_up(&mut self, user: &'a User) {
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
        let count_lajvarna = self.players.iter()
            .filter(|p| p.team == Team::Lajvarna).count();
        let count_gottarna = self.players.iter()
            .filter(|p| p.team == Team::Gottarna).count();

        if count_lajvarna <= count_gottarna {
            Team::Lajvarna
        } else {
            Team::Gottarna
        }
    }

    pub fn start_game(&self) -> Result<Game, GameError> {
        if self.ready_count() != 4 {
            return Err(GameError::RequiresFourReadyPlayers);
        }

        if !self.balanced_teams() {
            return Err(GameError::UnbalancedTeams);
        }

        let table = Table::new(self);

        Ok(
            Game::new(
                self.settings,
                table,
            )
        )
    }
}



#[cfg(test)]
mod tests {
    use crate::game::{Settings, Team};
    use super::*;

    fn setup_lobby() -> Lobby<'static> {
        let settings = Settings { to_win: 13 };
        Lobby::new(settings)
    }

    #[test]
    fn accepts_user_joining() {
        let mut game_lobby = setup_lobby();
        let user = User::new("John Doe");

        game_lobby.add_user(&user);

        assert_eq!(game_lobby.players.len(), 1);
    }

    #[test]
    fn denies_player_joining_twice() {
        let mut game_lobby = setup_lobby();
        let user = User::new("John Doe");

        game_lobby.add_user(&user);
        game_lobby.add_user(&user);

        assert_eq!(game_lobby.players.len(), 1);
    }

    #[test]
    fn accepts_user_leaving() {
        let mut game_lobby = setup_lobby();
        let user = User::new("John Doe");
        let user2 = User::new("Jane Doe");

        game_lobby.add_user(&user);
        game_lobby.add_user(&user2);
        game_lobby.del_user(&user);

        assert_eq!(game_lobby.players.len(), 1);
    }


    #[test]
    fn denies_user_leaving_twice() {
        let mut game_lobby = setup_lobby();
        let user = User::new("John Doe");
        let user2 = User::new("Jane Doe");

        game_lobby.add_user(&user);
        game_lobby.add_user(&user2);
        game_lobby.del_user(&user);
        game_lobby.del_user(&user);

        assert_eq!(game_lobby.players.len(), 1);
    }

    #[test]
    fn assigning_users_evenly_between_teams() {
        let mut game_lobby = setup_lobby();
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
    fn denies_5th_player() {
        let mut game_lobby = setup_lobby();
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
    fn allowing_players_to_change_team() {
        let mut game_lobby = setup_lobby();
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
        let mut game_lobby = setup_lobby();
        let user = User::new("John Doe");

        game_lobby.add_user(&user);
        game_lobby.ready_up(&user);

        assert_eq!(game_lobby.ready_count(), 1);
    }

    #[test]
    fn readying_up_twice_doesnt_change_anything() {
        let mut game_lobby = setup_lobby();
        let user = User::new("John Doe");

        game_lobby.add_user(&user);
        game_lobby.ready_up(&user);
        game_lobby.ready_up(&user);

        assert_eq!(game_lobby.ready_count(), 1);
    }

    #[test]
    fn player_can_unready() {
        let mut game_lobby = setup_lobby();
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
        let mut game_lobby = setup_lobby();
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
    fn cant_start_with_unbalanced_teams() {
        let mut game_lobby = setup_lobby();
        let user = User::new("John Doe");
        let user2 = User::new("Jane Doe");
        let user3 = User::new("Dolly");
        let user4 = User::new("Homer");

        game_lobby.add_user(&user);
        game_lobby.add_user(&user2);
        game_lobby.add_user(&user3);
        game_lobby.add_user(&user4);
        game_lobby.change_team(&user, Team::Lajvarna);
        game_lobby.change_team(&user2, Team::Gottarna);
        game_lobby.change_team(&user3, Team::Lajvarna);
        game_lobby.change_team(&user4, Team::Lajvarna);
        game_lobby.ready_up(&user);
        game_lobby.ready_up(&user2);
        game_lobby.ready_up(&user3);
        game_lobby.ready_up(&user4);

        assert_eq!(game_lobby.start_game(), Err(GameError::UnbalancedTeams));
    }

    #[test]
    fn can_start_with_four_ready_players() {
        let mut game_lobby = setup_lobby();
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

        let result = game_lobby.start_game();

        assert!(result.is_ok());
    }
}
