use uuid::Uuid;

use crate::{Error, Game, GetGame};
use std::sync::Arc;

pub trait ListGames {
    #[allow(async_fn_in_trait)]
    async fn list_games(&self) -> Result<Vec<Game>, Error>;
}

pub struct GetGameParameters<G>
where
    G: GetGame,
{
    pub games: Arc<G>,
    pub game_id: Uuid,
}

pub struct ListGamesParameters<G>
where
    G: ListGames,
{
    pub games: Arc<G>,
}

pub async fn get_game<G>(parameters: GetGameParameters<G>) -> Result<Game, Error>
where
    G: GetGame,
{
    let GetGameParameters { games, game_id } = parameters;

    games.get_game(game_id).await
}

pub async fn list_games<G>(parameters: ListGamesParameters<G>) -> Result<Vec<Game>, Error>
where
    G: ListGames,
{
    parameters.games.list_games().await
}
