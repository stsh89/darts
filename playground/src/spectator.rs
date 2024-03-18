use crate::{Error, Game, GameState, GetGameState};
use uuid::Uuid;

pub trait ListGames {
    #[allow(async_fn_in_trait)]
    async fn list_games(&self) -> Result<Vec<Game>, Error>;
}

pub struct GetGameParameters<'a, G>
where
    G: GetGameState,
{
    pub games: &'a G,
    pub game_id: Uuid,
}

pub struct ListGamesParameters<'a, G>
where
    G: ListGames,
{
    pub games: &'a G,
}

pub async fn get_game_state<G>(parameters: GetGameParameters<'_, G>) -> Result<GameState, Error>
where
    G: GetGameState,
{
    let GetGameParameters { games, game_id } = parameters;

    games.get_game_state(game_id).await
}

pub async fn list_games<G>(parameters: ListGamesParameters<'_, G>) -> Result<Vec<Game>, Error>
where
    G: ListGames,
{
    let games = parameters.games.list_games().await?;

    Ok(games)
}
