use crate::{Error, Number, PlayerScore};
use std::cmp::Ordering;
use uuid::Uuid;

#[derive(Clone)]
pub struct Round {
    id: Option<Uuid>,
    number: Number,
    player_number: Number,
    player_score: PlayerScore,
}

pub struct LoadRoundParameters {
    pub id: Uuid,
    pub number: Number,
    pub player_number: Number,
    pub player_score: PlayerScore,
}

pub struct NewRoundParameters {
    pub number: Number,
    pub player_number: Number,
    pub player_score: PlayerScore,
}

impl Eq for Round {}

impl PartialEq for Round {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.player_number == other.player_number
    }
}

impl PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Round {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.number.cmp(&other.number) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.player_number.cmp(&other.player_number)
    }
}

impl Round {
    pub fn assign_id(&mut self, id: Uuid) {
        self.id = Some(id);
    }

    pub fn id(&self) -> Option<Uuid> {
        self.id
    }

    pub fn is_persisted(&self) -> bool {
        self.id.is_some()
    }

    pub fn load(parameters: LoadRoundParameters) -> Result<Self, Error> {
        let LoadRoundParameters {
            id,
            player_number,
            player_score,
            number,
        } = parameters;

        Ok(Self {
            id: Some(id),
            player_number,
            player_score,
            number,
        })
    }

    pub fn new(parameters: NewRoundParameters) -> Result<Self, Error> {
        let NewRoundParameters {
            number,
            player_number,
            player_score,
        } = parameters;

        Ok(Self {
            id: None,
            player_number,
            player_score,
            number,
        })
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
