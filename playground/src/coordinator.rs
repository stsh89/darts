use crate::{Error, Game, NewGameParameters, Number, PlayerScore, Points, Score};
use uuid::Uuid;

pub trait GetGame {
    #[allow(async_fn_in_trait)]
    async fn get_game(&self, game_id: Uuid) -> Result<Game, Error>;
}

pub trait ListGames {
    #[allow(async_fn_in_trait)]
    async fn list_games(&self) -> Result<Vec<Game>, Error>;
}

pub trait SaveGame {
    #[allow(async_fn_in_trait)]
    async fn save_game(&self, game: &Game) -> Result<Uuid, Error>;
}

pub struct CountScoreParameters<'a, G>
where
    G: GetGame + SaveGame,
{
    pub game_id: Uuid,
    pub score: Score,
    pub games: &'a G,
}

pub struct GetGameParameters<'a, G>
where
    G: GetGame,
{
    pub game_id: Uuid,
    pub games: &'a G,
}

pub struct ListGamesParameters<'a, G>
where
    G: ListGames,
{
    pub games: &'a G,
}

pub struct SaveGameParameters {
    pub game_id: Uuid,
    pub player_number: i32,
    pub player_score: PlayerScore,
    pub round_number: u8,
}

pub struct StartGameParameters<'a, G>
where
    G: SaveGame,
{
    pub games: &'a G,
    pub players_number: Number,
    pub points_limit: Points,
}

pub async fn count_score<G>(parameters: CountScoreParameters<'_, G>) -> Result<Game, Error>
where
    G: GetGame + SaveGame,
{
    let CountScoreParameters {
        game_id,
        score,
        games,
    } = parameters;

    let mut game = games.get_game(game_id).await?;

    game.count_score(score)?;
    games.save_game(&game).await?;

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

pub async fn start_game<G>(parameters: StartGameParameters<'_, G>) -> Result<Game, Error>
where
    G: SaveGame,
{
    let StartGameParameters {
        games,
        players_number,
        points_limit,
    } = parameters;

    let mut game = Game::new(NewGameParameters {
        players_number,
        points_limit,
    })?;

    let id = games.save_game(&game).await?;
    game.assign_id(id);

    Ok(game)
}
