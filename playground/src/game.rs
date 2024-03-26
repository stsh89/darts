use std::collections::BTreeSet;

use crate::{Error, NewRoundParameters, Number, PlayerScore, Points, Round, Score};
use chrono::{DateTime, Utc};
use uuid::Uuid;

const PLAYERS_NUMBER: usize = 2;
const POINTS_LIMIT: u16 = 301;

pub struct Game {
    id: Option<Uuid>,
    players_number: Number,
    points_limit: Points,
    rounds: BTreeSet<Round>,
    start_time: Option<DateTime<Utc>>,
}

pub struct RoundPreview {
    player_number: Number,
    points_limit: Points,
    points: Points,
    round_number: Number,
}

impl RoundPreview {
    pub fn player_number(&self) -> Number {
        self.player_number
    }

    pub fn points_limit(&self) -> Points {
        self.points_limit
    }

    pub fn points(&self) -> Points {
        self.points
    }

    pub fn round_number(&self) -> Number {
        self.round_number
    }

    pub fn points_to_win(&self) -> Points {
        Points::new(self.points_limit.value() - self.points.value())
    }
}

pub struct PlayerStats {
    player_number: Number,
    points: Points,
    points_limit: Points,
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

    pub fn points_limit(&self) -> Points {
        self.points_limit
    }

    pub fn points_to_win(&self) -> Points {
        Points::new(self.points_limit.value() - self.points.value())
    }
}

pub struct LoadGameParameters {
    pub id: Uuid,
    pub rounds: Vec<Round>,
    pub start_time: DateTime<Utc>,
}

pub struct NewGameParameters {
    pub players_number: Number,
    pub points_limit: Points,
}

impl Game {
    fn assign_points_limit(&mut self, points_limit: Points) -> Result<(), Error> {
        if points_limit.is_zero() {
            return Err(Error::InvalidArgument(
                "Points limit cannot be zero".to_string(),
            ));
        }

        self.points_limit = points_limit;

        Ok(())
    }

    fn assign_players_number(&mut self, players_number: Number) {
        self.players_number = players_number;
    }

    fn assign_rounds(&mut self, rounds: Vec<Round>) {
        self.rounds = BTreeSet::from_iter(rounds);
    }

    pub fn count_score(&mut self, score: Score) -> Result<(), Error> {
        let Some(round_preview) = self.round_preview() else {
            return Err(Error::FailedPrecondition(
                "Cannot count a score when Game is over".to_string(),
            ));
        };

        let player_score = if score.points() > round_preview.points_to_win() {
            PlayerScore::overthrow(score)
        } else {
            PlayerScore::regular(score)
        };

        let round = Round::new(NewRoundParameters {
            number: round_preview.round_number,
            player_number: round_preview.player_number,
            player_score,
        })?;

        if self.rounds.insert(round) {
            Ok(())
        } else {
            Err(Error::AlreadyExists("Round already exists".to_string()))
        }
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

        let mut game = Self::new(NewGameParameters {
            players_number: unsafe { Number::new_unchecked(PLAYERS_NUMBER) },
            points_limit: Points::new(POINTS_LIMIT),
        })?;

        game.assign_id(id);
        game.assign_rounds(rounds);
        game.assign_start_time(start_time);

        Ok(game)
    }

    pub fn new(parameters: NewGameParameters) -> Result<Self, Error> {
        let NewGameParameters {
            points_limit,
            players_number,
        } = parameters;

        let mut game = Self::template();

        game.assign_players_number(players_number);
        game.assign_points_limit(points_limit)?;

        Ok(game)
    }

    pub fn players_stats(&self) -> Vec<PlayerStats> {
        let mut acc: Vec<PlayerStats> = (1..=self.players_number.value())
            .map(|player_number| PlayerStats {
                player_number: unsafe { Number::new_unchecked(player_number) },
                points: Points::zero(),
                points_limit: self.points_limit,
            })
            .collect();

        for round in self.rounds.iter() {
            let stats = acc.get_mut(round.player_number().value() - 1).unwrap();
            stats.add_points(round.player_score().game_points());
        }

        acc
    }

    fn players_number(&self) -> Number {
        self.players_number
    }

    pub fn round_preview(&self) -> Option<RoundPreview> {
        if self.is_over() {
            return None;
        }

        let Some(round) = self.rounds().last() else {
            return Some(RoundPreview {
                round_number: Number::one(),
                player_number: Number::one(),
                points: Points::zero(),
                points_limit: self.points_limit,
            });
        };

        let mut round_number = round.number();
        let mut player_number = round.player_number();

        if player_number == self.players_number() {
            round_number.increment();
            player_number = Number::one();
        } else {
            player_number.increment();
        }

        let points: Points = self
            .rounds
            .iter()
            .filter(|round| round.player_number() == player_number)
            .map(|round| round.player_score().game_points())
            .sum();

        if points == self.points_limit {
            return None;
        }

        Some(RoundPreview {
            round_number,
            player_number,
            points_limit: self.points_limit,
            points,
        })
    }

    pub fn rounds(&self) -> &BTreeSet<Round> {
        &self.rounds
    }

    pub fn assign_id(&mut self, id: Uuid) {
        self.id = Some(id);
    }

    pub fn assign_start_time(&mut self, start_time: DateTime<Utc>) {
        self.start_time = Some(start_time);
    }

    pub fn start_time(&self) -> Option<DateTime<Utc>> {
        self.start_time
    }

    fn template() -> Self {
        Self {
            id: None,
            points_limit: Points::zero(),
            players_number: Number::one(),
            rounds: BTreeSet::new(),
            start_time: None,
        }
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
