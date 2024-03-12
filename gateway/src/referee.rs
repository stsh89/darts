use crate::Error;
use score_tracker::{self, AddScore, GameScore};
use std::sync::Arc;
use uuid::Uuid;

pub use score_tracker::{PlayerScore, Score};

pub trait CreateGame {
    #[allow(async_fn_in_trait)]
    async fn create_game(&self) -> Result<Game, Error>;
}

pub trait GetGame {
    #[allow(async_fn_in_trait)]
    async fn find_game(&self, game_id: Uuid) -> Result<Game, Error>;
}

pub trait SaveScore {
    #[allow(async_fn_in_trait)]
    async fn save_score(&self, parameters: SaveScoreParameters) -> Result<(), Error>;
}

pub trait DeleteScore {
    #[allow(async_fn_in_trait)]
    async fn delete_score(&self, parameters: DeleteScoreParameters) -> Result<(), Error>;
}

pub struct DeleteScoreParameters<'a> {
    pub game_id: Uuid,
    pub player_name: &'a str,
    pub turn_number: u8,
}

pub struct Game {
    pub id: Uuid,
    pub player_number: PlayerNumber,
    pub player1_scores: Vec<PlayerScore>,
    pub player2_scores: Vec<PlayerScore>,
}

pub enum PlayerNumber {
    One,
    Two,
}

impl PlayerNumber {
    fn next(&mut self) {
        match self {
            PlayerNumber::One => PlayerNumber::Two,
            PlayerNumber::Two => PlayerNumber::One,
        };
    }

    fn previous(&mut self) {
        match self {
            PlayerNumber::One => PlayerNumber::Two,
            PlayerNumber::Two => PlayerNumber::One,
        };
    }

    pub fn name(&self) -> &str {
        match self {
            PlayerNumber::One => "Player1",
            PlayerNumber::Two => "Player2",
        }
    }
}

pub struct CountScoreParameters<G, S>
where
    G: GetGame,
    S: SaveScore,
{
    pub game_id: Uuid,
    pub score: Score,
    pub games: Arc<G>,
    pub scores: Arc<S>,
}

pub struct CancelScoreParameters<G, S>
where
    G: GetGame,
    S: DeleteScore,
{
    pub game_id: Uuid,
    pub games: Arc<G>,
    pub scores: Arc<S>,
}

pub struct SaveScoreParameters {
    pub game_id: Uuid,
    pub player_name: String,
    pub score: u8,
    pub turn_number: u8,
}

pub struct NewGameParameters<G>
where
    G: CreateGame,
{
    pub games: Arc<G>,
}

pub async fn new_game<G>(parameters: NewGameParameters<G>) -> Result<Game, Error>
where
    G: CreateGame,
{
    let NewGameParameters { games } = parameters;

    let game = games.create_game().await?;

    Ok(game)
}

pub async fn count_score<G, S>(parameters: CountScoreParameters<G, S>) -> Result<Game, Error>
where
    G: GetGame,
    S: SaveScore,
{
    let CountScoreParameters {
        game_id,
        score,
        games,
        scores,
    } = parameters;

    let mut game = games.find_game(game_id).await?;
    let score_to_save = score.value();

    let (player_name, turn_number) = match game.player_number {
        PlayerNumber::One => {
            game.player1_scores.add_score(score, &GameScore::new(301));
            ("Player1".to_string(), game.player1_scores.len() + 1)
        }
        PlayerNumber::Two => {
            game.player2_scores.add_score(score, &GameScore::new(301));
            ("Player2".to_string(), game.player2_scores.len() + 1)
        }
    };

    game.player_number.next();

    scores
        .save_score(SaveScoreParameters {
            game_id,
            player_name,
            score: score_to_save,
            turn_number: turn_number.try_into().map_err(eyre::Report::new)?,
        })
        .await?;

    Ok(game)
}

pub async fn cancel_score<G, S>(parameters: CancelScoreParameters<G, S>) -> Result<Game, Error>
where
    G: GetGame,
    S: DeleteScore,
{
    let CancelScoreParameters {
        game_id,
        games,
        scores,
    } = parameters;

    let mut game = games.find_game(game_id).await?;

    let (player_name, turn_number) = match game.player_number {
        PlayerNumber::One => {
            let data = ("Player1".to_string(), game.player1_scores.len() + 1);
            game.player1_scores.pop();
            data
        }
        PlayerNumber::Two => {
            let data = ("Player2".to_string(), game.player2_scores.len() + 1);
            game.player2_scores.pop();
            data
        }
    };

    scores
        .delete_score(DeleteScoreParameters {
            game_id,
            player_name: &player_name,
            turn_number: turn_number.try_into().map_err(eyre::Report::new)?,
        })
        .await?;

    game.player_number.previous();

    Ok(game)
}
