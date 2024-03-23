use crate::{PlayerScore, Points, PointsLimit, PositiveInteger, Score};

#[derive(Clone)]
pub struct Player {
    number: PositiveInteger,
    points_limit: PointsLimit,
    points: Points,
    scores: Vec<PlayerScore>,
}

pub struct NewPlayerParameters {
    pub number: PositiveInteger,
    pub points_limit: PointsLimit,
}

impl Player {
    pub fn add_score(&mut self, score: Score) {
        if self.is_winner() {
            return;
        }

        let player_score = if self.is_overthrow(score) {
            PlayerScore::Overthrow(score)
        } else {
            PlayerScore::Regular(score)
        };

        if player_score.is_regular() {
            self.points = self.points + score.points();
        }

        self.scores.push(player_score);
    }

    pub fn is_winner(&self) -> bool {
        self.points == self.points_limit.points()
    }

    pub fn new(parameters: NewPlayerParameters) -> Self {
        let NewPlayerParameters {
            points_limit,
            number,
        } = parameters;

        Self {
            number,
            points_limit,
            points: Points::zero(),
            scores: Vec::with_capacity(20),
        }
    }

    pub fn number(&self) -> PositiveInteger {
        self.number
    }

    /// Total number of `score` points. Overthrow points are not included.
    pub fn points(&self) -> Points {
        self.points
    }

    pub fn points_to_win(&self) -> Points {
        let points_limit: u16 = self.points_limit.points().into();
        let points: u16 = self.points.into();

        Points::from(points_limit - points)
    }

    pub fn last_score(&self) -> Option<&PlayerScore> {
        self.scores.last()
    }

    pub fn round_number(&self) -> usize {
        self.scores.len()
    }

    fn is_overthrow(&self, score: Score) -> bool {
        let total = self.points + score.points();

        total > self.points_limit.points()
    }

    pub fn scores(&self) -> &[PlayerScore] {
        self.scores.as_slice()
    }
}
