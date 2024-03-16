use tonic::{Request, Response, Status};

pub mod rpc {
    tonic::include_proto!("proto.playground"); // The string specified here must match the proto package name
}

pub struct Server {}

#[tonic::async_trait]
impl rpc::games_server::Games for Server {
    async fn cancel_last_score(
        &self,
        request: Request<rpc::CancelLastScoreRequest>,
    ) -> Result<Response<rpc::CancelLastScoreResponse>, Status> {
        let rpc::CancelLastScoreRequest { game_id: _ } = request.into_inner();

        Ok(Response::new(rpc::CancelLastScoreResponse {
            game_details: None,
        }))
    }

    async fn count_points(
        &self,
        request: Request<rpc::CountPointsRequest>,
    ) -> Result<Response<rpc::CountPointsResponse>, Status> {
        let rpc::CountPointsRequest {
            game_id: _,
            points: _,
        } = request.into_inner();

        Ok(Response::new(rpc::CountPointsResponse {
            game_details: None,
        }))
    }

    async fn create_game(
        &self,
        _request: Request<rpc::CreateGameRequest>,
    ) -> Result<Response<rpc::Game>, Status> {
        Ok(Response::new(rpc::Game::default()))
    }

    async fn get_game_details(
        &self,
        request: Request<rpc::GetGameDetailsRequest>,
    ) -> Result<Response<rpc::GameDetails>, Status> {
        let rpc::GetGameDetailsRequest { game_id: _ } = request.into_inner();

        Ok(Response::new(rpc::GameDetails::default()))
    }

    async fn list_games(
        &self,
        _request: Request<rpc::ListGamesRequest>,
    ) -> Result<Response<rpc::ListGamesResponse>, Status> {
        Ok(Response::new(rpc::ListGamesResponse::default()))
    }
}

impl Server {
    pub fn new() -> Self {
        Self {}
    }
}
