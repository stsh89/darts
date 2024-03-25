use std::ops::RangeInclusive;

use crate::{NewPlayerParameters, Number, Player, Score};

pub struct ScoreTracker {
    players_number: Number,
    players: Vec<Player>,
    points_limit: Number,
}

pub struct NewScoreTrackerParameters {
    pub players_number: Number,
    pub points_limit: Number,
}

impl ScoreTracker {
    fn initialize_players(&mut self) {
        let mut players = Vec::with_capacity(self.players_number.value());
        let range = RangeInclusive::new(1, self.players_number.value());

        for i in range {
            players.push(Player::new(NewPlayerParameters {
                number: unsafe { Number::new_unchecked(i) },
                points_limit: self.points_limit,
            }));
        }

        self.players = players
    }

    pub fn new(parameters: NewScoreTrackerParameters) -> Self {
        let NewScoreTrackerParameters {
            players_number,
            points_limit,
        } = parameters;

        let mut score_tracker = ScoreTracker {
            players_number,
            players: vec![],
            points_limit,
        };

        score_tracker.initialize_players();
        score_tracker
    }

    pub fn player(&self) -> &Player {
        self.players
            .iter()
            .min_by(|a, b| a.scores().len().cmp(&b.scores().len()))
            .expect("Error when trying to access player")
    }

    pub fn track(&mut self, score: Score) -> &Player {
        let player = self
            .players
            .iter_mut()
            .min_by(|a, b| a.scores().len().cmp(&b.scores().len()))
            .expect("Error when trying to access player_mut");

        player.add_score(score);

        player
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn points_limit(&self) -> Number {
        self.points_limit
    }

    pub fn winner(&self) -> Option<&Player> {
        self.players.iter().find(|p| p.is_winner())
    }
}
