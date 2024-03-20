mod error;
mod game_state;
mod player_number;
mod schedule;
mod score_details;

pub mod referee;
pub mod score_tracker;
pub mod spectator;

pub use error::Error;
pub use game_state::{GameState, LoadGameStateParameters, PlayerState, Round, Turn};
pub use player_number::PlayerNumber;
pub use schedule::{GamePreview, Schedule};
pub use score_details::{LoadScoreDetailsParameters, ScoreDetails};
pub use score_tracker::{AddScore, GameScore, PlayerScore, Score, TotalGameScore};

use uuid::Uuid;

pub trait GetGameState {
    #[allow(async_fn_in_trait)]
    async fn get_game_state(&self, game_id: Uuid) -> Result<GameState, Error>;
}
