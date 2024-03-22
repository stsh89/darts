use crate::PlayerScore;
use uuid::Uuid;

pub struct Round {
    id: Uuid,
    player_number: usize,
    player_score: PlayerScore,
    number: u8,
}

pub struct LoadRoundParameters {
    pub id: Uuid,
    pub player_number: usize,
    pub player_score: PlayerScore,
    pub number: u8,
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

    pub fn number(&self) -> u8 {
        self.number
    }

    pub fn player_number(&self) -> usize {
        self.player_number
    }

    pub fn player_score(&self) -> PlayerScore {
        self.player_score
    }
}
