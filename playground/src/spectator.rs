use crate::{Error, Game, GamePreview, GetGameState, Schedule};
use uuid::Uuid;

pub trait ListGamePreviews {
    #[allow(async_fn_in_trait)]
    async fn list_game_previews(&self) -> Result<Vec<GamePreview>, Error>;
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
    G: ListGamePreviews,
{
    pub games: &'a G,
}

pub async fn get_game_state<G>(parameters: GetGameParameters<'_, G>) -> Result<Game, Error>
where
    G: GetGameState,
{
    let GetGameParameters { games, game_id } = parameters;

    games.get_game_state(game_id).await
}

pub async fn get_schedule<G>(parameters: ListGamesParameters<'_, G>) -> Result<Schedule, Error>
where
    G: ListGamePreviews,
{
    let game_previews = parameters.games.list_game_previews().await?;

    Ok(Schedule::new(game_previews))
}
