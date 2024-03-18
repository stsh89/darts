use crate::{PlayerNumber, PlayerScore};
use uuid::Uuid;

pub struct ScoreDetails {
    id: Uuid,
    player_number: PlayerNumber,
    player_score: PlayerScore,
}

pub struct LoadScoreDetailsParameters {
    pub game_id: Uuid,
    pub id: Uuid,
    pub player_number: PlayerNumber,
    pub player_score: PlayerScore,
    pub turn_number: u8,
}

impl ScoreDetails {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn load(parameters: LoadScoreDetailsParameters) -> Self {
        let LoadScoreDetailsParameters {
            game_id: _,
            id,
            player_number,
            player_score,
            turn_number: _,
        } = parameters;

        Self {
            id,
            player_number,
            player_score,
        }
    }

    pub fn player_number(&self) -> PlayerNumber {
        self.player_number
    }

    pub fn player_score(&self) -> PlayerScore {
        self.player_score
    }
}
