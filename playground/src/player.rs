use crate::{Number, PlayerScore, Points, Score};

pub struct Player {
    number: Number,
    points_limit: Number,
    points: Points,
    scores: Vec<PlayerScore>,
}

pub struct NewPlayerParameters {
    pub number: Number,
    pub points_limit: Number,
}

impl Player {
    pub fn add_score(&mut self, score: Score) {
        if self.is_winner() {
            return;
        }

        let player_score = if self.is_overthrow(&score) {
            PlayerScore::Overthrow(score)
        } else {
            PlayerScore::Regular(score)
        };

        self.points = self.points + player_score.game_points();

        self.scores.push(player_score);
    }

    pub fn is_winner(&self) -> bool {
        self.points == Points::new(self.points_limit.value() as u16)
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

    pub fn number(&self) -> Number {
        self.number
    }

    /// Total number of `score` points. Overthrow points are not included.
    pub fn points(&self) -> Points {
        self.points
    }

    pub fn points_to_win(&self) -> Points {
        let points_limit: u16 = self.points_limit.value() as u16;
        let points: u16 = self.points.into();

        Points::from(points_limit - points)
    }

    pub fn last_score(&self) -> Option<&PlayerScore> {
        self.scores.last()
    }

    fn is_overthrow(&self, score: &Score) -> bool {
        let total = self.points + score.points();

        total > Points::new(self.points_limit.value() as u16)
    }

    pub fn scores(&self) -> &[PlayerScore] {
        self.scores.as_slice()
    }
}
