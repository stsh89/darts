use crate::{Error, Number, PlayerScore, Points, Score};
use chrono::{DateTime, Utc};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use uuid::Uuid;

pub struct Game {
    end_time: Option<DateTime<Utc>>,
    id: Option<Uuid>,
    players_number: Number,
    points_limit: Points,
    rounds: BTreeSet<Round>,
    start_time: Option<DateTime<Utc>>,
    state: State,
}

pub struct Round {
    number: Number,
    player_number: Number,
    player_score: PlayerScore,
}

pub struct NewRoundParameters {
    pub number: Number,
    pub player_number: Number,
    pub player_score: PlayerScore,
}

impl Round {
    pub fn new(parameters: NewRoundParameters) -> Self {
        let NewRoundParameters {
            number,
            player_number,
            player_score,
        } = parameters;

        Self {
            number,
            player_number,
            player_score,
        }
    }
}

pub struct LoadGameParameters {
    pub id: Uuid,
    pub rounds: Vec<Round>,
    pub end_time: Option<DateTime<Utc>>,
    pub start_time: Option<DateTime<Utc>>,
    pub players_number: Number,
    pub points_limit: Points,
}

pub struct NewGameParameters {
    pub players_number: Number,
    pub points_limit: Points,
}

impl Game {
    pub fn count_score(&mut self, score: Score) -> Result<(), Error> {
        match &self.state {
            State::NotStarted(state) => {
                let points = score.points();
                let mut player_number = state.player_number();
                let mut points_to_win = state.points_to_win();
                let mut round_number = Number::one();

                let player_score = if score.points() > self.points_limit {
                    PlayerScore::overthrow(score)
                } else {
                    PlayerScore::regular(score)
                };

                let insert_result = self.rounds.insert(Round {
                    number: Number::one(),
                    player_number: state.player_number(),
                    player_score,
                });

                if !insert_result {
                    return Error::unexpected(
                        "Fatal error while round insert. Game is not started.",
                    )
                    .into();
                }

                if points == points_to_win {
                    self.state = State::Finished(FinishedState {
                        winner: player_number,
                    });

                    self.end_time = Some(Utc::now());

                    return Ok(());
                }

                if self.players_number == Number::one() {
                    points_to_win = Points::new(points_to_win.value() - points.value());
                    round_number = unsafe { Number::new_unchecked(2) };
                } else {
                    player_number = unsafe { Number::new_unchecked(2) };
                }

                self.start_time = Some(Utc::now());

                self.state = State::InProgress(InProgressState {
                    player_number,
                    points_to_win,
                    round_number,
                });

                Ok(())
            }
            State::Finished(_) => {
                Error::failed_precondition("Cannot count a score when Game is over").into()
            }
            State::InProgress(state) => {
                let points = score.points();

                let total_points: Points = self
                    .rounds
                    .iter()
                    .filter(|r| r.player_number == state.player_number)
                    .map(|r| r.player_score.game_points())
                    .sum();
                let new_total_points = total_points + points;

                let player_score = if new_total_points > self.points_limit {
                    PlayerScore::overthrow(score)
                } else {
                    PlayerScore::regular(score)
                };

                let insert_result = self.rounds.insert(Round {
                    number: state.round_number,
                    player_number: state.player_number,
                    player_score,
                });

                if !insert_result {
                    return Error::unexpected(
                        "Fatal error while round insert. Game is in progress.",
                    )
                    .into();
                }

                if new_total_points == self.points_limit {
                    self.state = State::Finished(FinishedState {
                        winner: state.player_number,
                    });

                    self.end_time = Some(Utc::now());

                    return Ok(());
                }

                let mut player_number = state.player_number;
                let mut round_number = state.round_number;

                if state.player_number == self.players_number {
                    round_number = unsafe { Number::new_unchecked(round_number.value() + 1) };
                    player_number = Number::one();
                } else {
                    player_number = unsafe { Number::new_unchecked(player_number.value() + 1) };
                }

                let points: Points = self
                    .rounds
                    .iter()
                    .filter(|r| r.player_number == player_number)
                    .map(|r| r.player_score.game_points())
                    .sum();

                let points_to_win = Points::new(self.points_limit.value() - points.value());

                self.state = State::InProgress(InProgressState {
                    player_number,
                    points_to_win,
                    round_number,
                });

                Ok(())
            }
        }
    }

    pub fn load(parameters: LoadGameParameters) -> Result<Self, Error> {
        let LoadGameParameters {
            id,
            rounds,
            end_time,
            start_time,
            players_number,
            points_limit,
        } = parameters;

        let state = State::NotStarted(NotStartedState {
            points_to_win: points_limit,
        });

        let mut game = GameBuilder::new()
            .players_number(players_number)
            .points_limit(points_limit)
            .start_time(start_time)
            .id(Some(id))
            .end_time(end_time)
            .rounds(vec![])
            .state(state)
            .build()?;

        for round in rounds {
            game.count_score(Score::new(round.player_score().points().value())?)?;
        }

        Ok(game)
    }

    pub fn id(&self) -> Option<Uuid> {
        self.id
    }

    pub fn new(parameters: NewGameParameters) -> Result<Self, Error> {
        let NewGameParameters {
            points_limit,
            players_number,
        } = parameters;

        let state = State::NotStarted(NotStartedState {
            points_to_win: points_limit,
        });

        GameBuilder::new()
            .players_number(players_number)
            .points_limit(points_limit)
            .start_time(None)
            .id(None)
            .end_time(None)
            .rounds(vec![])
            .state(state)
            .build()
    }

    pub fn players_number(&self) -> Number {
        self.players_number
    }

    pub fn rounds(&self) -> &BTreeSet<Round> {
        &self.rounds
    }

    pub(crate) fn assign_id(&mut self, id: Uuid) {
        self.id = Some(id);
    }

    pub fn start_time(&self) -> Option<DateTime<Utc>> {
        self.start_time
    }

    pub fn end_time(&self) -> Option<DateTime<Utc>> {
        self.end_time
    }

    pub fn points_limit(&self) -> Points {
        self.points_limit
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn winner(&self) -> Option<Number> {
        match &self.state {
            State::Finished(state) => Some(state.winner),
            _ => None,
        }
    }
}

struct GameBuilder {
    id: Result<Option<Uuid>, Error>,
    players_number: Result<Number, Error>,
    points_limit: Result<Points, Error>,
    rounds: Result<BTreeSet<Round>, Error>,
    start_time: Result<Option<DateTime<Utc>>, Error>,
    end_time: Result<Option<DateTime<Utc>>, Error>,
    state: Result<State, Error>,
}

impl GameBuilder {
    fn build(self) -> Result<Game, Error> {
        Ok(Game {
            end_time: self.end_time?,
            id: self.id?,
            players_number: self.players_number?,
            points_limit: self.points_limit?,
            rounds: self.rounds?,
            start_time: self.start_time?,
            state: self.state?,
        })
    }

    fn new() -> Self {
        let error = |field: &str| Error::unexpected(format!("Game {field} not set"));

        Self {
            end_time: error("end time").into(),
            id: error("id").into(),
            players_number: error("players number").into(),
            points_limit: error("points limit").into(),
            rounds: error("rounds").into(),
            start_time: error("start time").into(),
            state: error("state").into(),
        }
    }

    fn id(mut self, id: Option<Uuid>) -> Self {
        self.id = Ok(id);

        self
    }

    fn players_number(mut self, players_number: Number) -> Self {
        self.players_number = Ok(players_number);

        self
    }

    fn points_limit(mut self, points_limit: Points) -> Self {
        self.points_limit = if points_limit.is_zero() {
            Error::invalid_argument("Points limit cannot be zero").into()
        } else {
            Ok(points_limit)
        };

        self
    }

    fn rounds(mut self, rounds: Vec<Round>) -> Self {
        self.rounds = Ok(BTreeSet::from_iter(rounds));

        self
    }

    fn start_time(mut self, start_time: Option<DateTime<Utc>>) -> Self {
        self.start_time = Ok(start_time);

        self
    }

    fn end_time(mut self, end_time: Option<DateTime<Utc>>) -> Self {
        self.end_time = Ok(end_time);

        self
    }

    fn state(mut self, state: State) -> Self {
        self.state = Ok(state);

        self
    }
}

impl Eq for Round {}

impl PartialEq for Round {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.player_number == other.player_number
    }
}

impl PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Round {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.number.cmp(&other.number) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.player_number.cmp(&other.player_number)
    }
}

impl Round {
    pub fn number(&self) -> Number {
        self.number
    }

    pub fn player_number(&self) -> Number {
        self.player_number
    }

    pub fn player_score(&self) -> &PlayerScore {
        &self.player_score
    }
}

pub enum State {
    NotStarted(NotStartedState),
    InProgress(InProgressState),
    Finished(FinishedState),
}

pub struct InProgressState {
    player_number: Number,
    round_number: Number,
    points_to_win: Points,
}

pub struct NotStartedState {
    points_to_win: Points,
}

pub struct FinishedState {
    winner: Number,
}

impl FinishedState {
    pub fn winner(&self) -> Number {
        self.winner
    }
}

impl InProgressState {
    pub fn player_number(&self) -> Number {
        self.player_number
    }

    pub fn points_to_win(&self) -> Points {
        self.points_to_win
    }
}

impl NotStartedState {
    pub fn player_number(&self) -> Number {
        Number::one()
    }

    pub fn points_to_win(&self) -> Points {
        self.points_to_win
    }
}
