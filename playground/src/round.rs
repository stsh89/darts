use crate::{PlayerScore, PositiveInteger};
use uuid::Uuid;

pub struct Round {
    id: Uuid,
    player_number: PositiveInteger,
    player_score: PlayerScore,
    number: PositiveInteger,
}

pub struct LoadRoundParameters {
    pub id: Uuid,
    pub player_number: PositiveInteger,
    pub player_score: PlayerScore,
    pub number: PositiveInteger,
}

impl Round {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn load(parameters: LoadRoundParameters) -> Self {
        let LoadRoundParameters {
            id,
            player_number,
            player_score,
            number,
        } = parameters;

        Self {
            id,
            player_number,
            player_score,
            number,
        }
    }

    pub fn number(&self) -> PositiveInteger {
        self.number
    }

    pub fn player_number(&self) -> PositiveInteger {
        self.player_number
    }

    pub fn player_score(&self) -> PlayerScore {
        self.player_score
    }
}
