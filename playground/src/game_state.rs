use crate::{Error, NewScoreTrackerParameters, Number, Points, PointsLimit, Round, ScoreTracker};
use uuid::Uuid;

const PLAYERS_NUMBER: usize = 2;
const POINTS_LIMIT: u16 = 301;

pub struct GameState {
    game_id: Uuid,
    rounds: Vec<Round>,
}

pub struct LoadGameStateParameters {
    pub game_id: Uuid,
    pub rounds: Vec<Round>,
}

impl GameState {
    pub fn add_round(&mut self, score_details: Round) {
        self.rounds.push(score_details);
    }

    pub fn game_id(&self) -> Uuid {
        self.game_id
    }

    pub fn load(parameters: LoadGameStateParameters) -> Result<Self, Error> {
        let LoadGameStateParameters { game_id, rounds } = parameters;

        Ok(Self { game_id, rounds })
    }

    pub fn remove_last_round(&mut self) -> Option<Round> {
        self.rounds.pop()
    }

    pub fn rounds(&self) -> &[Round] {
        &self.rounds
    }

    pub fn score_tracker(&self) -> ScoreTracker {
        let mut score_tracker = ScoreTracker::new(NewScoreTrackerParameters {
            players_number: players_number(),
            points_limit: points_limit(),
        });

        self.rounds().iter().for_each(|round| {
            score_tracker.track(*round.player_score().score());
        });

        score_tracker
    }
}

fn players_number() -> Number {
    Number::new(PLAYERS_NUMBER).expect("Positive integer should be more than 0")
}

fn points_limit() -> PointsLimit {
    PointsLimit::try_from(Points::from(POINTS_LIMIT)).expect("Points limit should be more than 0")
}
