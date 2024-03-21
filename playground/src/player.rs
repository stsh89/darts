use crate::{PlayerScore, Points, Score};

pub struct Player {
    number: usize,
    points_limit: Points,
    points: Points,
    scores: Vec<PlayerScore>,
}

pub struct NewPlayerParameters {
    pub number: usize,
    pub points_limit: Points,
}

impl Player {
    pub fn add_score(&mut self, score: Score) {
        let player_score = self.get_player_score(score);

        if player_score.is_score() {
            self.points = self.points + score.points();
        }

        self.scores.push(player_score);
    }

    pub fn points(&self) -> Points {
        self.points
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

    fn score_overthrow(&self, score: Score) -> bool {
        (self.points + score.points()) > self.points_limit
    }

    pub fn new(parameters: NewPlayerParameters) -> Self {
        let NewPlayerParameters {
            points_limit,
            number,
        } = parameters;

        Self {
            number,
            points_limit,
            points: Points::from(0),
            scores: Vec::with_capacity(20),
        }
    }

    pub fn number(&self) -> usize {
        self.number
    }

    pub fn points_to_win(&self) -> Points {
        let points_limit: u16 = self.points_limit.into();
        let points: u16 = self.points.into();

        Points::from(points_limit - points)
    }
}
