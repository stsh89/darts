use crate::playground::rpc;
use chrono::{DateTime, Utc};
use itertools::Itertools;
use playground::{Error, GamePreview, GameState, Player, PlayerScore, Round};
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

impl ToRpc<rpc::Game> for GamePreview {
    fn to_rpc(self) -> rpc::Game {
        rpc::Game {
            id: self.game_id().to_string(),
            start_time: Some(self.start_time().to_rpc()),
        }
    }
}

impl ToRpc<rpc::GameDetails> for GameState {
    fn to_rpc(self) -> rpc::GameDetails {
        let score_tracker = self.score_tracker();
        let player = score_tracker.player();

        rpc::GameDetails {
            game_id: self.game_id().to_string(),
            winner: score_tracker
                .winner()
                .map(|player| format!("Player{}", player.number() + 1))
                .unwrap_or_default(),
            player: format!("Player{}", player.number() + 1),
            player_points_to_win: player.points_to_win().into(),
            rounds: rounds(&self),
            player_details: score_tracker.players().iter().map(ToRpc::to_rpc).collect(),
        }
    }
}

impl ToRpc<rpc::PlayerDetails> for &Player {
    fn to_rpc(self) -> rpc::PlayerDetails {
        rpc::PlayerDetails {
            name: format!("Player{}", self.number() + 1),
            points_to_win: self.points_to_win().into(),
        }
    }
}

impl ToRpc<rpc::Point> for PlayerScore {
    fn to_rpc(self) -> rpc::Point {
        match self {
            PlayerScore::Regular(score) => rpc::Point {
                kind: rpc::PointKind::Score.into(),
                value: score.points().into(),
            },
            PlayerScore::Overthrow(score) => rpc::Point {
                kind: rpc::PointKind::Overthrow.into(),
                value: score.points().into(),
            },
        }
    }
}

// impl ToRpc<rpc::Round> for Round {
//     fn to_rpc(self) -> rpc::Round {
//         rpc::Round {
//             number: self.number,
//             points: self,
//         }
//     }
// }

impl ToRpc<Status> for Error {
    fn to_rpc(self) -> Status {
        match self {
            Error::FailedPrecondition(description) => Status::failed_precondition(description),
            Error::InvalidArgument(description) => Status::invalid_argument(description),
            Error::NotFound(description) => Status::not_found(description),
            Error::Repo(report) => Status::internal(report.to_string()),
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

// pub fn rounds(game_state: &GameState) -> Vec<Round> {
//     let rounds_number = game_state
//         .players()
//         .first()
//         .map(|p| p.scores().len())
//         .unwrap_or(0);
//     let mut rounds: Vec<Round> = Vec::with_capacity(rounds_number);

//     for round_number in 0..rounds_number {
//         for player in game_state.players() {
//             if let Some(score) = player.scores().get(round_number) {
//                 if let Some(round) = rounds.get_mut(round_number) {
//                     round.player_scores.push(*score);
//                 } else {
//                     rounds.push(Round {
//                         number: round_number as u8,
//                         player_scores: vec![*score],
//                     });
//                 }
//             }
//         }
//     }

//     rounds
// }

pub fn rounds(game_state: &GameState) -> Vec<rpc::Round> {
    let groups: HashMap<u8, Vec<&Round>> = game_state
        .rounds()
        .iter()
        .into_grouping_map_by(|r| r.number())
        .collect();

    let mut rounds: Vec<rpc::Round> = groups
        .into_iter()
        .map(|(number, round)| rpc::Round {
            number: number.into(),
            points: round
                .iter()
                .map(|data| data.player_score().to_rpc())
                .collect(),
        })
        .collect();

    rounds.sort_by(|a, b| b.number.cmp(&a.number));
    rounds

    // let players_number = 2;

    // //TODO: calculate capacity in a more efficient way.
    // let mut rounds = Vec::with_capacity(50);
    // let mut round_number = 1;

    // for chunk in game_state.rounds().chunks(players_number) {
    //     let mut round = rpc::Round {
    //         number: round_number,
    //         points: Vec::with_capacity(players_number),
    //     };

    //     for data in chunk {
    //         round.points.push(data.player_score().to_rpc());
    //     }

    //     rounds.push(round);
    //     round_number += 1;
    // }

    // rounds
}
