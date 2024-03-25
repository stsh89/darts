use crate::{Error, NewScoreTrackerParameters, Number, Round, ScoreTracker};
use uuid::Uuid;

const PLAYERS_NUMBER: usize = 2;
const POINTS_LIMIT: usize = 301;

pub struct Game {
    id: Uuid,
    rounds: Vec<Round>,
}

pub struct LoadGameStateParameters {
    pub game_id: Uuid,
    pub rounds: Vec<Round>,
}

impl Game {
    pub fn add_round(&mut self, score_details: Round) {
        self.rounds.push(score_details);
    }

    pub fn game_id(&self) -> Uuid {
        self.id
    }

    pub fn load(parameters: LoadGameStateParameters) -> Result<Self, Error> {
        let LoadGameStateParameters { game_id, rounds } = parameters;

        Ok(Self {
            id: game_id,
            rounds,
        })
    }

    pub fn remove_last_round(&mut self) -> Option<Round> {
        self.rounds.pop()
    }

    pub fn rounds(&self) -> &[Round] {
        &self.rounds
    }

    pub fn score_tracker(&self) -> ScoreTracker {
        let mut score_tracker = ScoreTracker::new(NewScoreTrackerParameters {
            players_number: unsafe { Number::new_unchecked(PLAYERS_NUMBER) },
            points_limit: unsafe { Number::new_unchecked(POINTS_LIMIT) },
        });

        self.rounds().iter().for_each(|round| {
            score_tracker.track(*round.player_score().score());
        });

        score_tracker
    }
}
