use crate::{Points, Score};

pub enum PlayerScore {
    Regular(Score),
    Overthrow(Score),
}

impl PlayerScore {
    pub fn game_points(&self) -> Points {
        match self {
            PlayerScore::Regular(score) => score.points(),
            PlayerScore::Overthrow(_) => Points::zero(),
        }
    }

    pub fn is_regular(&self) -> bool {
        matches!(self, PlayerScore::Regular(_))
    }

    pub fn is_overthrow(&self) -> bool {
        matches!(self, PlayerScore::Overthrow(_))
    }

    pub fn overthrow(score: Score) -> Self {
        PlayerScore::Overthrow(score)
    }

    pub fn regular(score: Score) -> Self {
        PlayerScore::Regular(score)
    }

    pub fn score(&self) -> &Score {
        match self {
            PlayerScore::Regular(score) => score,
            PlayerScore::Overthrow(score) => score,
        }
    }
}
