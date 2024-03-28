use crate::convert::{ToRpc, TryConvert};
use dataspine::Repo;
use playground::{self, coordinator, Number, Points, Score};
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
        // Ok(Response::new(rpc::CancelLastScoreResponse::default()))
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn count_points(
        &self,
        request: Request<rpc::CountPointsRequest>,
    ) -> Result<Response<rpc::CountPointsResponse>, Status> {
        let rpc::CountPointsRequest { game_id, points } = request.into_inner();

        let score = Score::new(points as u16).map_err(ToRpc::to_rpc)?;

        let game = coordinator::count_score(coordinator::CountScoreParameters {
            games: &self.repo,
            game_id: game_id.try_convert()?,
            score,
        })
        .await
        .map_err(ToRpc::to_rpc)?;

        Ok(Response::new(rpc::CountPointsResponse {
            game: Some(game.to_rpc()),
        }))
    }

    async fn create_game(
        &self,
        _request: Request<rpc::CreateGameRequest>,
    ) -> Result<Response<rpc::Game>, Status> {
        let game = coordinator::initialize_game(coordinator::InitializeGameParameters {
            players_number: Number::new(2).unwrap(),
            points_limit: Points::new(301),
            games: &self.repo,
        })
        .await
        .map_err(ToRpc::to_rpc)?;

        Ok(Response::new(game.to_rpc()))
    }

    async fn get_game(
        &self,
        request: Request<rpc::GetGameRequest>,
    ) -> Result<Response<rpc::Game>, Status> {
        let rpc::GetGameRequest { game_id } = request.into_inner();

        let game = coordinator::get_game(coordinator::GetGameParameters {
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
            coordinator::list_games(coordinator::ListGamesParameters { games: &self.repo })
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
