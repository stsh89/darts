use crate::{Error, Game, NewGameParameters, Number, Points, Score};
use uuid::Uuid;

pub trait GetGame {
    #[allow(async_fn_in_trait)]
    async fn get_game(&self, game_id: Uuid) -> Result<Game, Error>;
}

pub trait InsertGame {
    #[allow(async_fn_in_trait)]
    async fn insert_game(&self, game: &mut Game) -> Result<(), Error>;
}

pub trait ListGames {
    #[allow(async_fn_in_trait)]
    async fn list_games(&self) -> Result<Vec<Game>, Error>;
}

pub trait UpdateGame {
    #[allow(async_fn_in_trait)]
    async fn update_game(&self, game: &Game) -> Result<(), Error>;
}

pub struct CountScoreParameters<'a, G>
where
    G: GetGame + UpdateGame,
{
    pub game_id: Uuid,
    pub games: &'a G,
    pub score: Score,
}

pub struct GetGameParameters<'a, G>
where
    G: GetGame,
{
    pub game_id: Uuid,
    pub games: &'a G,
}

pub struct InitializeGameParameters<'a, G>
where
    G: InsertGame,
{
    pub games: &'a G,
    pub players_number: Number,
    pub points_limit: Points,
}

pub struct ListGamesParameters<'a, G>
where
    G: ListGames,
{
    pub games: &'a G,
}

pub async fn count_score<G>(parameters: CountScoreParameters<'_, G>) -> Result<Game, Error>
where
    G: GetGame + UpdateGame,
{
    let CountScoreParameters {
        game_id,
        score,
        games,
    } = parameters;

    let mut game = games.get_game(game_id).await?;

    game.count_score(score)?;
    games.update_game(&game).await?;

    Ok(game)
}

pub async fn initialize_game<G>(parameters: InitializeGameParameters<'_, G>) -> Result<Game, Error>
where
    G: InsertGame,
{
    let InitializeGameParameters {
        games,
        players_number,
        points_limit,
    } = parameters;

    let mut game = Game::new(NewGameParameters {
        players_number,
        points_limit,
    })?;

    games.insert_game(&mut game).await?;

    Ok(game)
}

pub async fn get_game<G>(parameters: GetGameParameters<'_, G>) -> Result<Game, Error>
where
    G: GetGame,
{
    let GetGameParameters { games, game_id } = parameters;

    let game = games.get_game(game_id).await?;

    Ok(game)
}

pub async fn list_games<G>(parameters: ListGamesParameters<'_, G>) -> Result<Vec<Game>, Error>
where
    G: ListGames,
{
    let games = parameters.games.list_games().await?;

    Ok(games)
}
