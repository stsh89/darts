pub mod referee;
pub mod repo;
pub mod spectator;

pub use score_tracker::{GameScore, PlayerScore, TotalGameScore};

use uuid::Uuid;

pub trait GetGame {
    #[allow(async_fn_in_trait)]
    async fn get_game(&self, game_id: Uuid) -> Result<Game, Error>;
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("data access error: {0}")]
    DataAccess(#[source] eyre::Report),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("unexpected error: {0}")]
    UnexpectedError(#[from] eyre::Report),
}

pub fn max_game_score() -> GameScore {
    GameScore::new(301)
}

pub struct Game {
    pub id: Uuid,
    pub player_number: PlayerNumber,
    pub player1_scores: Vec<PlayerScore>,
    pub player2_scores: Vec<PlayerScore>,
}

impl PlayerNumber {
    fn next(&mut self) {
        *self = match self {
            PlayerNumber::One => PlayerNumber::Two,
            PlayerNumber::Two => PlayerNumber::One,
        };
    }

    fn previous(&mut self) {
        *self = match self {
            PlayerNumber::One => PlayerNumber::Two,
            PlayerNumber::Two => PlayerNumber::One,
        };
    }

    pub fn name(&self) -> &str {
        match self {
            PlayerNumber::One => "Player1",
            PlayerNumber::Two => "Player2",
        }
    }
}

pub enum PlayerNumber {
    One,
    Two,
}
