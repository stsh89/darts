use std::collections::BTreeSet;

use crate::{
    Error, NewRoundParameters, NewScoreTrackerParameters, Number, Points, Round, Score,
    ScoreTracker,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

const PLAYERS_NUMBER: usize = 2;
const POINTS_LIMIT: usize = 301;

#[derive(Default)]
pub struct Game {
    id: Option<Uuid>,
    rounds: BTreeSet<Round>,
    start_time: Option<DateTime<Utc>>,
}

pub struct RoundPreview {
    pub round_number: Number,
    pub player_number: Number,
    pub points_to_win: Number,
}

pub struct PlayerStats {
    player_number: Number,
    points: Points,
    points_limit: Number,
}

impl PlayerStats {
    pub fn add_points(&mut self, points: Points) {
        self.points = self.points + points;
    }

    pub fn player_number(&self) -> Number {
        self.player_number
    }

    pub fn points(&self) -> Points {
        self.points
    }

    pub fn points_limit(&self) -> Number {
        self.points_limit
    }

    pub fn points_to_win(&self) -> Points {
        Points::new((self.points_limit.value() as u16) - self.points.value())
    }
}

pub struct LoadGameParameters {
    pub id: Uuid,
    pub rounds: Vec<Round>,
    pub start_time: DateTime<Utc>,
}

impl Game {
    pub fn count_score(&mut self, score: Score) -> Result<(), Error> {
        let mut score_tracker = self.score_tracker();
        let player = score_tracker.track(score);
        let round = Round::new(NewRoundParameters {
            number: unsafe { Number::new_unchecked(player.scores().len()) },
            player_number: player.number(),
            player_score: *player.last_score().unwrap(),
        })?;

        self.rounds.insert(round);

        Ok(())
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
            rounds: BTreeSet::from_iter(rounds),
            start_time: Some(start_time),
        })
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn players_stats(&self) -> Vec<PlayerStats> {
        let mut acc: Vec<PlayerStats> = (1..=PLAYERS_NUMBER)
            .map(|player_number| PlayerStats {
                player_number: unsafe { Number::new_unchecked(player_number) },
                points: Points::zero(),
                points_limit: unsafe { Number::new_unchecked(POINTS_LIMIT) },
            })
            .collect();

        for round in self.rounds.iter() {
            let stats = acc.get_mut(round.player_number().value() - 1).unwrap();
            stats.add_points(round.player_score().game_points());
        }

        acc
    }

    fn players_number(&self) -> Number {
        unsafe { Number::new_unchecked(PLAYERS_NUMBER) }
    }

    pub fn round_preview(&self) -> Option<RoundPreview> {
        if self.is_over() {
            return None;
        }

        let Some(round) = self.rounds().last() else {
            return Some(RoundPreview {
                round_number: Number::one(),
                player_number: Number::one(),
                points_to_win: unsafe { Number::new_unchecked(POINTS_LIMIT) },
            });
        };

        let mut round_number = round.number();
        let mut player_number = round.player_number();

        if player_number == self.players_number() {
            round_number.increment();
            player_number = Number::one();
        }

        let points_to_win: Points = self
            .rounds
            .iter()
            .filter(|round| round.player_number() == player_number)
            .map(|round| round.player_score().game_points())
            .sum();

        if points_to_win.is_zero() {
            return None;
        }

        Some(RoundPreview {
            round_number,
            player_number,
            points_to_win: unsafe { Number::new_unchecked(points_to_win.value().into()) },
        })
    }

    pub fn rounds(&self) -> &BTreeSet<Round> {
        &self.rounds
    }

    fn score_tracker(&self) -> ScoreTracker {
        let mut score_tracker = ScoreTracker::new(NewScoreTrackerParameters {
            players_number: unsafe { Number::new_unchecked(PLAYERS_NUMBER) },
            points_limit: unsafe { Number::new_unchecked(POINTS_LIMIT) },
        });

        self.rounds().iter().for_each(|round| {
            score_tracker.track(*round.player_score().score());
        });

        score_tracker
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = Some(id);
    }

    pub fn set_start_time(&mut self, start_time: DateTime<Utc>) {
        self.start_time = Some(start_time);
    }

    pub fn start_time(&self) -> Option<DateTime<Utc>> {
        self.start_time
    }

    pub fn winner(&self) -> Option<Number> {
        self.players_stats()
            .iter()
            .find(|stats| stats.points_to_win().is_zero())
            .map(|stats| stats.player_number())
    }

    fn is_over(&self) -> bool {
        self.winner().is_some()
    }
}
