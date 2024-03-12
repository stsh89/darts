use crate::{Error, Game};
use std::sync::Arc;

pub trait ListGames {
    #[allow(async_fn_in_trait)]
    async fn list_games(&self) -> Result<Vec<Game>, Error>;
}

pub struct ListGamesParameters<G>
where
    G: ListGames,
{
    pub games: Arc<G>,
}

pub async fn list_games<G>(parameters: ListGamesParameters<G>) -> Result<Vec<Game>, Error>
where
    G: ListGames,
{
    parameters.games.list_games().await
}
