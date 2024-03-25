use crate::{Number, PlayerScore};
use uuid::Uuid;

pub struct Round {
    id: Uuid,
    player_number: Number,
    player_score: PlayerScore,
    number: Number,
}

pub struct LoadRoundParameters {
    pub id: Uuid,
    pub player_number: Number,
    pub player_score: PlayerScore,
    pub number: Number,
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

    pub fn number(&self) -> Number {
        self.number
    }

    pub fn player_number(&self) -> Number {
        self.player_number
    }

    pub fn player_score(&self) -> PlayerScore {
        self.player_score
    }
}
