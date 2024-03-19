use chrono::{DateTime, Utc};
use dataspine::Repo;
use playground::{
    self,
    referee::{self, StartGameParameters},
    spectator, Error, Game, GameState, PlayerScore, PlayerState, Round, Score,
};
use std::time::SystemTime;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod rpc {
    tonic::include_proto!("proto.playground"); // The string specified here must match the proto package name
}

pub struct Server {
    repo: Repo,
}

#[tonic::async_trait]
impl rpc::games_server::Games for Server {
    async fn cancel_last_score(
        &self,
        request: Request<rpc::CancelLastScoreRequest>,
    ) -> Result<Response<rpc::CancelLastScoreResponse>, Status> {
        let rpc::CancelLastScoreRequest { game_id } = request.into_inner();

        let game_id = uuid(&game_id)?;
        let game_state = referee::cancel_last_score(referee::CancelLastScoreParameters {
            game_id,
            games: &self.repo,
            scores: &self.repo,
        })
        .await
        .map_err(status)?;
        let game_details = rpc_game_details(game_state);

        Ok(Response::new(rpc::CancelLastScoreResponse {
            game_details: Some(game_details),
        }))
    }

    async fn count_points(
        &self,
        request: Request<rpc::CountPointsRequest>,
    ) -> Result<Response<rpc::CountPointsResponse>, Status> {
        let rpc::CountPointsRequest { game_id, points } = request.into_inner();

        let game_id = uuid(&game_id)?;

        let points = TryInto::<u8>::try_into(points)
            .map_err(|_| Status::invalid_argument("Too many points"))?;
        let score = Score::new(points);

        let game_state = referee::count_score(referee::CountScoreParameters {
            games: &self.repo,
            game_id,
            score,
            scores: &self.repo,
        })
        .await
        .map_err(status)?;

        let game_details = rpc_game_details(game_state);

        Ok(Response::new(rpc::CountPointsResponse {
            game_details: Some(game_details),
        }))
    }

    async fn create_game(
        &self,
        _request: Request<rpc::CreateGameRequest>,
    ) -> Result<Response<rpc::Game>, Status> {
        let game = referee::start_game(StartGameParameters { games: &self.repo })
            .await
            .map_err(status)?;

        let game = rpc_game(game);

        Ok(Response::new(game))
    }

    async fn get_game_details(
        &self,
        request: Request<rpc::GetGameDetailsRequest>,
    ) -> Result<Response<rpc::GameDetails>, Status> {
        let rpc::GetGameDetailsRequest { game_id } = request.into_inner();

        let game_id = uuid(&game_id)?;
        let game_state = spectator::get_game_state(spectator::GetGameParameters {
            games: &self.repo,
            game_id,
        })
        .await
        .map_err(status)?;

        let game_details = rpc_game_details(game_state);

        Ok(Response::new(game_details))
    }

    async fn list_games(
        &self,
        _request: Request<rpc::ListGamesRequest>,
    ) -> Result<Response<rpc::ListGamesResponse>, Status> {
        let games = spectator::list_games(spectator::ListGamesParameters { games: &self.repo })
            .await
            .map_err(status)?;

        let games = games.into_iter().map(rpc_game).collect();

        Ok(Response::new(rpc::ListGamesResponse { games }))
    }
}

impl Server {
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }
}

fn rpc_game(game: Game) -> rpc::Game {
    rpc::Game {
        id: game.id().to_string(),
        start_time: Some(timestamp(game.start_time())),
    }
}

fn rpc_game_details(mut game_state: GameState) -> rpc::GameDetails {
    let current_player_state = game_state.current_player_state();

    rpc::GameDetails {
        game_id: game_state.game_id().to_string(),
        winner: game_state
            .winner()
            .map(|player| player.name().to_string())
            .unwrap_or_default(),
        player: current_player_state.player_number().name().to_string(),
        player_points_to_win: current_player_state.points_to_win().value().into(),
        rounds: game_state
            .rounds()
            .into_iter()
            .rev()
            .map(rpc_round)
            .collect(),
        player_details: game_state
            .players_game_scores()
            .into_iter()
            .map(rpc_player_details)
            .collect(),
    }
}

fn rpc_player_details(player_state: PlayerState) -> rpc::PlayerDetails {
    rpc::PlayerDetails {
        name: player_state.player_number().name().to_string(),
        points_to_win: player_state.points_to_win().value().into(),
    }
}

fn rpc_point(player_score: PlayerScore) -> rpc::Point {
    rpc::Point {
        kind: match player_score {
            PlayerScore::Score(_) => rpc::PointKind::Score,
            PlayerScore::Overthrow(_) => rpc::PointKind::Overthrow,
        }
        .into(),
        value: player_score.into_inner().into(),
    }
}

fn rpc_round(round: Round) -> rpc::Round {
    rpc::Round {
        number: round.number.into(),
        points: round.player_scores.into_iter().map(rpc_point).collect(),
    }
}

fn status(err: Error) -> Status {
    match err {
        Error::FailedPrecondition(description) => Status::failed_precondition(description),
        Error::NotFound(description) => Status::not_found(description),
        Error::Repo(report) => Status::internal(report.to_string()),
        Error::Unexpected(report) => Status::internal(report.to_string()),
    }
}

fn timestamp(datetime: DateTime<Utc>) -> prost_types::Timestamp {
    let systime: SystemTime = datetime.into();
    systime.into()
}

fn uuid(string: &str) -> Result<Uuid, Status> {
    Uuid::parse_str(string).map_err(|_err| Status::invalid_argument(format!("UUID: {}", string)))
}
