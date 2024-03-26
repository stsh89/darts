use crate::{Error, Game, PlayerScore, Score};
use uuid::Uuid;

pub trait GetGame {
    #[allow(async_fn_in_trait)]
    async fn get_game(&self, game_id: Uuid) -> Result<Game, Error>;
}

pub trait SaveGame {
    #[allow(async_fn_in_trait)]
    async fn save_game(&self, game: &mut Game) -> Result<(), Error>;
}

pub struct CountScoreParameters<'a, G>
where
    G: GetGame + SaveGame,
{
    pub game_id: Uuid,
    pub score: Score,
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
    games.save_game(&mut game).await?;

    Ok(game)
}

pub async fn start_game<G>(parameters: StartGameParameters<'_, G>) -> Result<Game, Error>
where
    G: SaveGame,
{
    let StartGameParameters { games } = parameters;

    let mut game = Game::new();
    games.save_game(&mut game).await?;

    Ok(game)
}
