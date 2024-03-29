use crate::{Error, Number, PlayerScore, Points, Score};
use chrono::{DateTime, Utc};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use uuid::Uuid;

pub struct Game {
    create_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    id: Option<Uuid>,
    players_number: Number,
    points_limit: Points,
    rounds: BTreeSet<Round>,
    start_time: Option<DateTime<Utc>>,
    state: State,
    update_time: Option<DateTime<Utc>>,
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

pub struct LoadGameParameters {
    pub create_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub id: Uuid,
    pub players_number: Number,
    pub points_limit: Points,
    pub rounds: Vec<Round>,
    pub start_time: Option<DateTime<Utc>>,
    pub update_time: DateTime<Utc>,
}

pub struct NewGameParameters {
    pub players_number: Number,
    pub points_limit: Points,
}

impl Game {
    pub fn assign_create_time(&mut self, create_time: DateTime<Utc>) -> Result<(), Error> {
        if self.create_time.is_some() {
            return Error::unexpected("Attempt to reassign game creation time").into();
        }

        self.create_time = Some(create_time);

        Ok(())
    }

    pub fn assign_id(&mut self, id: Uuid) -> Result<(), Error> {
        if self.id.is_some() {
            return Error::unexpected("Game ID reassignment attempt").into();
        }

        self.id = Some(id);

        Ok(())
    }

    fn assign_rounds(&mut self, rounds: Vec<Round>) -> Result<(), Error> {
        if !self.rounds.is_empty() {
            return Error::unexpected("Game rounds reassignment attempt").into();
        }

        for round in rounds {
            let points = round.player_score().score().points();
            let score = Score::new(points.value())?;

            self.count_score(score)?;
        }

        Ok(())
    }

    fn assign_start_time(&mut self, start_time: DateTime<Utc>) -> Result<(), Error> {
        if self.start_time.is_some() {
            return Error::failed_precondition("Attempt to reassign start time").into();
        }

        self.start_time = Some(start_time);

        Ok(())
    }

    fn assign_end_time(&mut self, end_time: DateTime<Utc>) -> Result<(), Error> {
        if self.end_time.is_some() {
            return Error::failed_precondition("Attempt to reassign end time").into();
        }

        if let Some(time) = self.start_time {
            if time > end_time {
                return Error::invalid_argument("End time cannot be less than start time").into();
            }
        } else {
            return Error::failed_precondition("Attempt to assign end time without start time")
                .into();
        }

        self.end_time = Some(end_time);

        Ok(())
    }

    fn change_players_number(&mut self, players_number: Number) -> Result<(), Error> {
        if self.is_in_progress() {
            return Error::failed_precondition(
                "Cannot update players number when game is in progress",
            )
            .into();
        };

        if self.is_finished() {
            return Error::failed_precondition("Cannot update players number when game is over")
                .into();
        };

        self.players_number = players_number;

        Ok(())
    }

    fn change_points_limit(&mut self, points_limit: Points) -> Result<(), Error> {
        if points_limit.is_zero() {
            return Error::invalid_argument("Points limit cannot be zero").into();
        };

        if self.is_in_progress() {
            return Error::failed_precondition(
                "Cannot update points limit when game is in progress",
            )
            .into();
        };

        if self.is_finished() {
            return Error::failed_precondition("Cannot update points limit when Game is over")
                .into();
        };

        self.points_limit = points_limit;

        Ok(())
    }

    pub fn change_update_time(&mut self, update_time: DateTime<Utc>) -> Result<(), Error> {
        if let Some(time) = self.update_time {
            if time > update_time {
                return Error::invalid_argument("Update time cannot be less than before").into();
            }
        }

        self.update_time = Some(update_time);

        Ok(())
    }

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

                    self.assign_end_time(Utc::now())?;

                    return Ok(());
                }

                if self.players_number == Number::one() {
                    points_to_win = Points::new(points_to_win.value() - points.value());
                    round_number = unsafe { Number::new_unchecked(2) };
                } else {
                    player_number = unsafe { Number::new_unchecked(2) };
                }

                self.assign_start_time(Utc::now())?;

                self.state = State::InProgress(InProgressState {
                    player_number,
                    points_to_win,
                    round_number,
                });

                Ok(())
            }
            State::Finished(_) => {
                Error::failed_precondition("Cannot count a score when game is over").into()
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

                    self.assign_end_time(Utc::now())?;

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

    pub fn create_time(&self) -> Option<DateTime<Utc>> {
        self.create_time
    }

    pub fn end_time(&self) -> Option<DateTime<Utc>> {
        self.end_time
    }

    pub fn id(&self) -> Option<Uuid> {
        self.id
    }

    fn init() -> Game {
        Game {
            create_time: None,
            end_time: None,
            id: None,
            players_number: Number::one(),
            points_limit: Points::zero(),
            rounds: BTreeSet::new(),
            start_time: None,
            state: State::not_started(Points::zero()),
            update_time: None,
        }
    }

    fn is_finished(&self) -> bool {
        matches!(self.state, State::Finished(_))
    }

    fn is_in_progress(&self) -> bool {
        matches!(self.state, State::InProgress(_))
    }

    pub fn load(parameters: LoadGameParameters) -> Result<Self, Error> {
        let LoadGameParameters {
            create_time,
            end_time: _,
            id,
            players_number,
            points_limit,
            rounds,
            start_time: _,
            update_time,
        } = parameters;

        let mut game = Self::new(NewGameParameters {
            points_limit,
            players_number,
        })?;

        game.assign_id(id)?;
        game.assign_create_time(create_time)?;
        game.assign_rounds(rounds)?;
        game.change_update_time(update_time)?;

        Ok(game)
    }

    pub fn new(parameters: NewGameParameters) -> Result<Self, Error> {
        let NewGameParameters {
            points_limit,
            players_number,
        } = parameters;

        let mut game = Self::init();

        game.change_points_limit(points_limit)?;
        game.change_players_number(players_number)?;

        Ok(game)
    }

    pub fn players_number(&self) -> Number {
        self.players_number
    }

    pub fn rounds(&self) -> &BTreeSet<Round> {
        &self.rounds
    }

    pub fn start_time(&self) -> Option<DateTime<Utc>> {
        self.start_time
    }

    pub fn points_limit(&self) -> Points {
        self.points_limit
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn update_time(&self) -> Option<DateTime<Utc>> {
        self.update_time
    }

    pub fn winner(&self) -> Option<Number> {
        match &self.state {
            State::Finished(state) => Some(state.winner),
            _ => None,
        }
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

impl State {
    pub(crate) fn not_started(points_to_win: Points) -> Self {
        Self::NotStarted(NotStartedState { points_to_win })
    }
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
