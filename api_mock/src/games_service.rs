use chrono::{DateTime, Utc};
use std::{collections::HashMap, time::SystemTime};
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod rpc {
    tonic::include_proto!("proto.playground"); // The string specified here must match the proto package name
}

pub struct GamesService {
    games: RwLock<HashMap<Uuid, Game>>,
}

#[derive(Clone)]
struct Game {
    create_time: DateTime<Utc>,
    id: Uuid,
    player_details: Vec<PlayerDetails>,
    player_points_to_win: u16,
    player: Player,
    rounds: Vec<Round>,
    scores: HashMap<String, Vec<u16>>,
    update_time: DateTime<Utc>,
    winner: Option<String>,
}

#[derive(Copy, Clone)]
enum Player {
    One,
    Two,
}

impl Player {
    fn name(&self) -> String {
        match self {
            Player::One => "Player1".to_string(),
            Player::Two => "Player2".to_string(),
        }
    }

    fn next(&mut self) {
        *self = match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

#[derive(Clone)]
struct Round {
    number: u16,
    points: Vec<Point>,
}

#[derive(Copy, Clone)]
enum Point {
    Score(u16),
    Overthrow(u16),
}

impl Point {
    fn into_inner(self) -> u16 {
        match self {
            Point::Score(score) => score,
            Point::Overthrow(points) => points,
        }
    }
}

#[derive(Clone)]
struct PlayerDetails {
    player: Player,
    points_to_win: u16,
}

#[tonic::async_trait]
impl rpc::games_server::Games for GamesService {
    async fn cancel_last_score(
        &self,
        request: Request<rpc::CancelLastScoreRequest>,
    ) -> Result<Response<rpc::CancelLastScoreResponse>, Status> {
        let rpc::CancelLastScoreRequest { game_id } = request.into_inner();

        let game_id = Uuid::parse_str(&game_id)
            .map_err(|_err| Status::invalid_argument("Invalid game id"))?;

        let mut games = self.games.write().await;
        let game = games.get_mut(&game_id);

        let Some(mut game) = game.cloned() else {
            return Err(Status::not_found("Game not found"));
        };

        let scores = game
            .scores
            .get_mut(&game.player.name())
            .expect("Player should have scores");
        scores.pop();

        game.player.next();

        game.update_player_details();
        game.remove_last_round_points();
        games.insert(game_id, game.clone());

        let proto = game
            .clone()
            .try_into()
            .map_err(|_err| Status::internal("Error"))?;

        Ok(Response::new(rpc::CancelLastScoreResponse {
            game: Some(proto),
        }))
    }

    async fn count_points(
        &self,
        request: Request<rpc::CountPointsRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<rpc::CountPointsResponse>, Status> {
        let rpc::CountPointsRequest { game_id, points } = request.into_inner();
        let game_id = Uuid::parse_str(&game_id)
            .map_err(|_err| Status::invalid_argument("Invalid game id"))?;

        let mut games = self.games.write().await;
        let game = games.get_mut(&game_id);

        let Some(mut game) = game.cloned() else {
            return Err(Status::not_found("Game not found"));
        };

        let scores = game
            .scores
            .get_mut(&game.player.name())
            .expect("Player should have scores");
        scores.push(points as u16);

        if scores.iter().sum::<u16>() == 301 {
            game.winner = Some(game.player.name());
        }
        game.player.next();

        game.update_player_details();
        game.update_rounds(points as u16);
        games.insert(game_id, game.clone());

        let proto = game
            .clone()
            .try_into()
            .map_err(|_err| Status::internal("Error"))?;

        Ok(Response::new(rpc::CountPointsResponse {
            game: Some(proto),
        }))
    }

    async fn create_game(
        &self,
        _request: Request<rpc::CreateGameRequest>,
    ) -> Result<Response<rpc::Game>, Status> {
        let game = Game {
            id: Uuid::new_v4(),
            winner: None,
            player: Player::One,
            player_points_to_win: 301,
            rounds: vec![],
            update_time: Utc::now(),
            create_time: Utc::now(),
            scores: HashMap::from_iter(vec![
                (Player::One.name(), vec![]),
                (Player::Two.name(), vec![]),
            ]),
            player_details: vec![
                PlayerDetails {
                    player: Player::One,
                    points_to_win: 301,
                },
                PlayerDetails {
                    player: Player::Two,
                    points_to_win: 301,
                },
            ],
        };

        let mut games = self.games.write().await;
        games.insert(game.id, game.clone());
        let proto: rpc::Game = game.try_into().map_err(|_err| Status::internal("Error"))?;

        Ok(Response::new(proto))
    }

    async fn get_game(
        &self,
        request: Request<rpc::GetGameRequest>,
    ) -> Result<Response<rpc::Game>, Status> {
        let rpc::GetGameRequest { game_id } = request.into_inner();
        let game_id = Uuid::parse_str(&game_id)
            .map_err(|_err| Status::invalid_argument("Invalid game id"))?;

        let games = self.games.read().await;
        let game = games.get(&game_id);

        let Some(game) = game.cloned() else {
            return Err(Status::not_found("Game not found"));
        };

        let proto = game.try_into().map_err(|_err| Status::internal("Error"))?;

        Ok(Response::new(proto))
    }

    async fn list_games(
        &self,
        _request: Request<rpc::ListGamesRequest>,
    ) -> Result<Response<rpc::ListGamesResponse>, Status> {
        let games = self.games.read().await;

        let proto = games
            .values()
            .cloned()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_err| Status::internal("Error"))?;

        Ok(Response::new(rpc::ListGamesResponse { games: proto }))
    }
}

impl GamesService {
    pub fn new() -> Self {
        Self {
            games: RwLock::new(HashMap::new()),
        }
    }
}

impl TryFrom<Game> for rpc::Game {
    type Error = anyhow::Error;

    fn try_from(value: Game) -> anyhow::Result<Self> {
        Ok(Self {
            id: value.id.to_string(),
            winner: value.winner.unwrap_or_default(),
            player: value.player.name(),
            player_points_to_win: value.player_points_to_win.into(),
            rounds: value.rounds.into_iter().rev().map(Into::into).collect(),
            player_details: value.player_details.into_iter().map(Into::into).collect(),
            create_time: Some(timestamp(value.create_time)),
            update_time: Some(timestamp(value.update_time)),
        })
    }
}

impl From<PlayerDetails> for rpc::PlayerDetails {
    fn from(value: PlayerDetails) -> Self {
        Self {
            name: value.player.name(),
            points_to_win: value.points_to_win.into(),
        }
    }
}

impl From<Round> for rpc::Round {
    fn from(value: Round) -> Self {
        Self {
            number: value.number.into(),
            points: value.points.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<Point> for rpc::Point {
    fn from(value: Point) -> Self {
        Self {
            value: value.into_inner().into(),
            kind: match value {
                Point::Score(_) => rpc::PointKind::Regular.into(),
                Point::Overthrow(_) => rpc::PointKind::Overthrow.into(),
            },
        }
    }
}

impl Game {
    fn update_player_details(&mut self) {
        let player1_scores = self
            .scores
            .get(&Player::One.name())
            .expect("Player 1 should have scores");
        let player2_scores = self
            .scores
            .get(&Player::Two.name())
            .expect("Player 2 should have scores");

        self.player_details = vec![
            PlayerDetails {
                player: Player::One,
                points_to_win: 301 - player1_scores.iter().sum::<u16>(),
            },
            PlayerDetails {
                player: Player::Two,
                points_to_win: 301 - player2_scores.iter().sum::<u16>(),
            },
        ];

        match self.player {
            Player::One => self.player_points_to_win = 301 - player1_scores.iter().sum::<u16>(),
            Player::Two => self.player_points_to_win = 301 - player2_scores.iter().sum::<u16>(),
        }
    }

    pub fn remove_last_round_points(&mut self) {
        let round = self.rounds.last_mut();

        if let Some(round) = round {
            if round.points.len() == 2 {
                round.points.pop();
            } else {
                self.rounds.pop();
            }
        }
    }

    pub fn update_rounds(&mut self, points: u16) {
        let len = self.rounds.len();
        let round = self.rounds.last_mut();

        let point = if points > 100 {
            Point::Overthrow(points)
        } else {
            Point::Score(points)
        };

        if let Some(round) = round {
            if round.points.len() == 2 {
                self.rounds.push(Round {
                    number: len as u16 + 1,
                    points: vec![point],
                });
            } else {
                round.points.push(point);
            }
        } else {
            self.rounds.push(Round {
                number: 1,
                points: vec![point],
            });
        }
    }
}

fn timestamp(datetime: DateTime<Utc>) -> prost_types::Timestamp {
    let systime: SystemTime = datetime.into();
    systime.into()
}
