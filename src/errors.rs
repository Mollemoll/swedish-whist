#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameError {
    RequiresFourReadyPlayers,
    UnbalancedTeams,
}
