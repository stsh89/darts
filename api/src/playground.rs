use crate::convert::{ToRpc, TryConvert};
use dataspine::Repo;
use playground::{
    self,
    referee::{self, StartGameParameters},
    spectator, Score,
};
use tonic::{Request, Response, Status};

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
        _request: Request<rpc::CancelLastScoreRequest>,
    ) -> Result<Response<rpc::CancelLastScoreResponse>, Status> {
        Ok(Response::new(rpc::CancelLastScoreResponse::default()))
    }

    async fn count_points(
        &self,
        request: Request<rpc::CountPointsRequest>,
    ) -> Result<Response<rpc::CountPointsResponse>, Status> {
        let rpc::CountPointsRequest { game_id, points } = request.into_inner();

        let score = Score::new(points as u16).map_err(ToRpc::to_rpc)?;

        let game_state = referee::count_score(referee::CountScoreParameters {
            games: &self.repo,
            game_id: game_id.try_convert()?,
            score,
        })
        .await
        .map_err(ToRpc::to_rpc)?;

        Ok(Response::new(rpc::CountPointsResponse {
            game_details: Some(game_state.to_rpc()),
        }))
    }

    async fn create_game(
        &self,
        _request: Request<rpc::CreateGameRequest>,
    ) -> Result<Response<rpc::Game>, Status> {
        let game = referee::start_game(StartGameParameters { games: &self.repo })
            .await
            .map_err(ToRpc::to_rpc)?;

        Ok(Response::new(rpc::Game {
            id: game.id().unwrap().to_string(),
            start_time: game.start_time().map(ToRpc::to_rpc),
        }))
    }

    async fn get_game_details(
        &self,
        request: Request<rpc::GetGameDetailsRequest>,
    ) -> Result<Response<rpc::GameDetails>, Status> {
        let rpc::GetGameDetailsRequest { game_id } = request.into_inner();

        let game = spectator::get_game(spectator::GetGameParameters {
            games: &self.repo,
            game_id: game_id.try_convert()?,
        })
        .await
        .map_err(ToRpc::to_rpc)?;

        Ok(Response::new(game.to_rpc()))
    }

    async fn list_games(
        &self,
        _request: Request<rpc::ListGamesRequest>,
    ) -> Result<Response<rpc::ListGamesResponse>, Status> {
        let game_previews =
            spectator::list_game_previews(spectator::ListGamesParameters { games: &self.repo })
                .await
                .map_err(ToRpc::to_rpc)?;

        let games = game_previews.into_iter().map(ToRpc::to_rpc).collect();

        Ok(Response::new(rpc::ListGamesResponse { games }))
    }
}

impl Server {
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }
}
