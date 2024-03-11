pub mod referee;
pub mod repo;

pub use score_tracker::{GameScore, PlayerScore};

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
