use crate::{PlayerScore, Points, Score};

pub struct Player {
    game_points: Points,
    limit: Points,
    number: usize,
    scores: Vec<PlayerScore>,
}

pub struct NewPlayerParameters {
    pub game_limit: Points,
    pub number: usize,
}

impl Player {
    pub fn add_game_points(&mut self, points: Points) {
        self.game_points = self.game_points + points;
    }

    pub fn add_score(&mut self, score: Score) {
        let player_score = self.get_player_score(score);

        if player_score.is_score() {
            self.game_points = self.game_points + score.points();
        }

        self.scores.push(player_score);
    }

    pub fn game_points(&self) -> Points {
        self.game_points
    }

    pub fn is_winner(&self) -> bool {
        self.game_points() == self.limit
    }

    fn get_player_score(&mut self, score: Score) -> PlayerScore {
        if self.score_overthrow(score) {
            PlayerScore::Overthrow(score)
        } else {
            PlayerScore::Score(score)
        }
    }

    fn score_overthrow(&self, score: Score) -> bool {
        (self.game_points() + score.points()) > self.limit
    }

    pub fn new(parameters: NewPlayerParameters) -> Self {
        let NewPlayerParameters {
            game_limit: limit,
            number,
        } = parameters;

        Self {
            game_points: Points::from(0),
            limit,
            number,
            scores: Vec::with_capacity(20),
        }
    }

    pub fn number(&self) -> usize {
        self.number
    }

    pub fn points_to_win(&self) -> Points {
        let limit: u16 = self.limit.into();
        let game_points: u16 = self.game_points.into();

        Points::from(limit - game_points)
    }
}
