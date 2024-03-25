use crate::{Error, GamePreview, GameState, GetGameState, PlayerScore, Round, Score};
use uuid::Uuid;

pub trait DeleteScore {
    #[allow(async_fn_in_trait)]
    async fn delete_score(&self, id: Uuid) -> Result<(), Error>;
}

pub trait InsertGamePreview {
    #[allow(async_fn_in_trait)]
    async fn insert_game_preview(&self) -> Result<GamePreview, Error>;
}

pub trait InsertScore {
    #[allow(async_fn_in_trait)]
    async fn insert_score(&self, parameters: InsertScoreParameters) -> Result<Round, Error>;
}

pub struct CancelLastScoreParameters<'a, G, S>
where
    G: GetGameState,
    S: DeleteScore,
{
    pub game_id: Uuid,
    pub games: &'a G,
    pub scores: &'a S,
}

pub struct CountScoreParameters<'a, G, S>
where
    G: GetGameState,
    S: InsertScore,
{
    pub game_id: Uuid,
    pub score: Score,
    pub games: &'a G,
    pub scores: &'a S,
}

pub struct InsertScoreParameters {
    pub game_id: Uuid,
    pub player_number: i32,
    pub player_score: PlayerScore,
    pub round_number: u8,
}

pub struct StartGameParameters<'a, G>
where
    G: InsertGamePreview,
{
    pub games: &'a G,
}

pub async fn cancel_last_score<G, S>(
    parameters: CancelLastScoreParameters<'_, G, S>,
) -> Result<GameState, Error>
where
    G: GetGameState,
    S: DeleteScore,
{
    let CancelLastScoreParameters {
        game_id,
        games,
        scores,
    } = parameters;

    let mut game_state = games.get_game_state(game_id).await?;

    let Some(round) = game_state.remove_last_round() else {
        return Err(Error::FailedPrecondition("Empty scores list".to_string()));
    };

    scores.delete_score(round.id()).await?;

    Ok(game_state)
}

pub async fn count_score<G, S>(
    parameters: CountScoreParameters<'_, G, S>,
) -> Result<GameState, Error>
where
    G: GetGameState,
    S: InsertScore,
{
    let CountScoreParameters {
        game_id,
        score,
        games,
        scores,
    } = parameters;

    let mut game_state = games.get_game_state(game_id).await?;

    let mut score_tracker = game_state.score_tracker();
    let player = score_tracker.track(score);

    let score_details = scores
        .insert_score(InsertScoreParameters {
            game_id,
            player_number: player
                .number()
                .value()
                .try_into()
                .map_err(Into::<eyre::Report>::into)?,
            player_score: *player
                .last_score()
                .ok_or(Error::FailedPrecondition("No last score".to_string()))?,
            round_number: player.round_number() as u8,
        })
        .await?;

    game_state.add_round(score_details);

    Ok(game_state)
}

pub async fn start_game<G>(parameters: StartGameParameters<'_, G>) -> Result<GamePreview, Error>
where
    G: InsertGamePreview,
{
    let StartGameParameters { games } = parameters;

    let game_preview = games.insert_game_preview().await?;

    Ok(game_preview)
}
