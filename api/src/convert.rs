use crate::playground::rpc;
use chrono::{DateTime, Utc};
use itertools::Itertools;
use playground::{Error, Game, PlayerScore, Round, State};
use prost_types::Timestamp;
use std::{collections::HashMap, time::SystemTime};
use tonic::Status;
use uuid::Uuid;

pub trait ToRpc<T> {
    fn to_rpc(self) -> T;
}

pub trait TryConvert<T> {
    fn try_convert(self) -> Result<T, Status>;
}

impl ToRpc<rpc::Game> for Game {
    fn to_rpc(self) -> rpc::Game {
        rpc::Game {
            id: self.id().unwrap().to_string(),
            start_time: self.start_time().map(ToRpc::to_rpc),
        }
    }
}

impl ToRpc<rpc::GameDetails> for Game {
    fn to_rpc(self) -> rpc::GameDetails {
        let state = self.state();

        let (player, player_points_to_win) = match state {
            State::NotStarted(state) => (
                format!("Player{}", state.player_number()),
                state.points_to_win().value().into(),
            ),
            State::InProgress(state) => (
                format!("Player{}", state.player_number()),
                state.points_to_win().value().into(),
            ),
            State::Finished(_) => ("".to_string(), 0),
        };

        rpc::GameDetails {
            game_id: self.id().unwrap().to_string(),
            winner: self
                .winner()
                .map(|number| format!("Player{}", number))
                .unwrap_or_default(),
            player,
            player_points_to_win,
            rounds: rounds(&self),
            player_details: player_details(&self),
        }
    }
}

impl ToRpc<rpc::Point> for &PlayerScore {
    fn to_rpc(self) -> rpc::Point {
        match self {
            PlayerScore::Regular(score) => rpc::Point {
                kind: rpc::PointKind::Score.into(),
                value: score.points().value().into(),
            },
            PlayerScore::Overthrow(score) => rpc::Point {
                kind: rpc::PointKind::Overthrow.into(),
                value: score.points().value().into(),
            },
        }
    }
}

impl ToRpc<Status> for Error {
    fn to_rpc(self) -> Status {
        match self {
            Error::AlreadyExists(description) => Status::already_exists(description),
            Error::FailedPrecondition(description) => Status::failed_precondition(description),
            Error::InvalidArgument(description) => Status::invalid_argument(description),
            Error::NotFound(description) => Status::not_found(description),
            Error::Unexpected(report) => Status::internal(report.to_string()),
        }
    }
}

impl ToRpc<Timestamp> for DateTime<Utc> {
    fn to_rpc(self) -> Timestamp {
        let systime: SystemTime = self.into();
        systime.into()
    }
}

impl TryConvert<Uuid> for String {
    fn try_convert(self) -> Result<Uuid, Status> {
        Uuid::parse_str(&self).map_err(|_err| Status::invalid_argument(format!("Uuid: {self}")))
    }
}

fn rounds(game: &Game) -> Vec<rpc::Round> {
    let groups: HashMap<usize, Vec<&Round>> = game
        .rounds()
        .iter()
        .into_grouping_map_by(|r| r.number().value())
        .collect();

    let mut rounds: Vec<(usize, Vec<&Round>)> = groups.into_iter().collect();
    rounds.sort_by_key(|(i, _r)| *i);
    rounds.iter_mut().for_each(|(_i, r)| r.sort_by_key(|ro| (ro.number(), ro.player_number())));

    let rounds: Vec<rpc::Round> = rounds
        .into_iter()
        .rev()
        .map(|(number, round)| rpc::Round {
            number: number.try_into().unwrap(),
            points: round
                .iter()
                .map(|data| data.player_score().to_rpc())
                .collect(),
        })
        .collect();

    rounds
}

fn player_details(game: &Game) -> Vec<rpc::PlayerDetails> {
    let groups: HashMap<usize, Vec<&Round>> = game
        .rounds()
        .iter()
        .into_grouping_map_by(|r| r.player_number().value())
        .collect();

    let mut player_details: Vec<rpc::PlayerDetails> = Vec::with_capacity(groups.len());

    for i in 0..game.players_number().value() {
        player_details.push(rpc::PlayerDetails {
            points_to_win: game.points_limit().value().into(),
            name: format!("Player{}", i + 1),
        });
    }

    for (player_number, rounds) in groups {
        if let Some(details) = player_details.get_mut(player_number - 1) {
            for round in rounds {
                details.points_to_win = match round.player_score() {
                    PlayerScore::Regular(score) => {
                        details.points_to_win - (score.points().value() as i32)
                    }
                    PlayerScore::Overthrow(_) => details.points_to_win,
                }
            }
        }
    }

    player_details
}
