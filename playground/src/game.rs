use crate::{Error, NewScoreTrackerParameters, Number, Round, ScoreTracker};
use chrono::{DateTime, Utc};
use uuid::Uuid;

const PLAYERS_NUMBER: usize = 2;
const POINTS_LIMIT: usize = 301;

#[derive(Default)]
pub struct Game {
    id: Option<Uuid>,
    rounds: Vec<Round>,
    start_time: Option<DateTime<Utc>>,
}

pub struct LoadGameParameters {
    pub id: Uuid,
    pub rounds: Vec<Round>,
    pub start_time: DateTime<Utc>,
}

impl Game {
    pub fn add_round(&mut self, score_details: Round) {
        self.rounds.push(score_details);
    }

    pub fn id(&self) -> Option<Uuid> {
        self.id
    }

    pub fn is_persisted(&self) -> bool {
        self.id.is_some()
    }

    pub fn load(parameters: LoadGameParameters) -> Result<Self, Error> {
        let LoadGameParameters {
            id,
            rounds,
            start_time,
        } = parameters;

        Ok(Self {
            id: Some(id),
            rounds,
            start_time: Some(start_time),
        })
    }

    pub fn new() -> Self {
        Self::default()
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

    pub fn start_time(&self) -> Option<DateTime<Utc>> {
        self.start_time
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = Some(id);
    }

    pub fn set_start_time(&mut self, start_time: DateTime<Utc>) {
        self.start_time = Some(start_time);
    }
}
