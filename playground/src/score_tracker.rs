use crate::{NewPlayerParameters, Player, Points, Score};

pub struct ScoreTracker {
    players_number: usize,
    players: Vec<Player>,
    points_limit: Points,
}

pub struct LoadScoreTrackerParameters {
    pub players_number: usize,
    pub players: Vec<Player>,
    pub points_limit: u16,
}

pub struct NewScoreTrackerParameters {
    pub players_number: usize,
    pub points_limit: u16,
}

impl ScoreTracker {
    fn initialize_players(&mut self) {
        let mut players = Vec::with_capacity(self.players_number);

        for number in 0..self.players_number {
            players.push(Player::new(NewPlayerParameters {
                number,
                points_limit: self.points_limit.into(),
            }));
        }

        self.players = players
    }

    pub fn load(parameters: LoadScoreTrackerParameters) -> Self {
        let LoadScoreTrackerParameters {
            players_number,
            players,
            points_limit,
        } = parameters;

        ScoreTracker {
            players_number,
            players,
            points_limit: Points::from(points_limit),
        }
    }

    pub fn new(parameters: NewScoreTrackerParameters) -> Self {
        let NewScoreTrackerParameters {
            players_number,
            points_limit,
        } = parameters;

        let mut score_tracker = ScoreTracker {
            players_number,
            players: vec![],
            points_limit: Points::from(points_limit),
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

    pub fn points_limit(&self) -> Points {
        self.points_limit
    }

    pub fn winner(&self) -> Option<&Player> {
        self.players.iter().find(|p| p.is_winner())
    }
}
