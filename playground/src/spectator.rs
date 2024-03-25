use uuid::Uuid;

use crate::{Error, Game, GamePreview};

pub trait GetGame {
    #[allow(async_fn_in_trait)]
    async fn get_game(&self, game_id: Uuid) -> Result<Game, Error>;
}

pub trait ListGamePreviews {
    #[allow(async_fn_in_trait)]
    async fn list_game_previews(&self) -> Result<Vec<GamePreview>, Error>;
}

pub struct GetGameParameters<'a, G>
where
    G: GetGame,
{
    pub games: &'a G,
    pub game_id: Uuid,
}

pub struct ListGamesParameters<'a, G>
where
    G: ListGamePreviews,
{
    pub games: &'a G,
}

pub async fn get_game<G>(parameters: GetGameParameters<'_, G>) -> Result<Game, Error>
where
    G: GetGame,
{
    let GetGameParameters { games, game_id } = parameters;

    let game = games.get_game(game_id).await?;

    Ok(game)
}

pub async fn list_game_previews<G>(
    parameters: ListGamesParameters<'_, G>,
) -> Result<Vec<GamePreview>, Error>
where
    G: ListGamePreviews,
{
    let game_previews = parameters.games.list_game_previews().await?;

    Ok(game_previews)
}
