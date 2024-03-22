use crate::{PlayerScore, Points, Score};

#[derive(Clone)]
pub struct Player {
    number: usize,
    points_limit: Points,
    points: Points,
    scores: Vec<PlayerScore>,
}

pub struct NewPlayerParameters {
    pub number: usize,
    pub points_limit: u16,
}

impl Player {
    pub fn add_score(&mut self, score: Score) {
        if self.is_winner() {
            return;
        }

        let player_score = self.get_player_score(score);

        if player_score.is_score() {
            self.points = self.points + score.points();
        }

        self.scores.push(player_score);
    }

    pub fn add_player_score(&mut self, player_score: PlayerScore) {
        if self.is_winner() {
            return;
        }

        if player_score.is_score() && ((self.points + player_score.points()) > self.points_limit) {
            return;
        }

        if player_score.is_overthrow()
            && ((self.points + player_score.points()) <= self.points_limit)
        {
            return;
        }

        self.scores.push(player_score);
    }

    pub fn is_winner(&self) -> bool {
        self.points == self.points_limit
    }

    fn get_player_score(&mut self, score: Score) -> PlayerScore {
        if self.score_overthrow(score) {
            PlayerScore::Overthrow(score)
        } else {
            PlayerScore::Score(score)
        }
    }

    pub fn new(parameters: NewPlayerParameters) -> Self {
        let NewPlayerParameters {
            points_limit,
            number,
        } = parameters;

        Self {
            number,
            points_limit: Points::from(points_limit),
            points: Points::from(0),
            scores: Vec::with_capacity(20),
        }
    }

    pub fn number(&self) -> usize {
        self.number
    }

    /// Total number of `score` points. Overthrow points are not included.
    pub fn points(&self) -> Points {
        self.points
    }

    pub fn points_to_win(&self) -> Points {
        let points_limit: u16 = self.points_limit.into();
        let points: u16 = self.points.into();

        Points::from(points_limit - points)
    }

    pub fn last_score(&self) -> Option<&PlayerScore> {
        self.scores.last()
    }

    pub fn round_number(&self) -> usize {
        self.scores.len()
    }

    fn score_overthrow(&self, score: Score) -> bool {
        (self.points + score.points()) > self.points_limit
    }

    pub fn scores(&self) -> &[PlayerScore] {
        self.scores.as_slice()
    }
}
