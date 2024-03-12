use std::sync::Arc;

use crate::referee::{DeleteScore, DeleteScoreParameters, SaveScoreParameters};
use crate::PlayerScore;
use dataspine::InsertScoreParameters;
use score_tracker::{AddScore, Score};
use uuid::Uuid;

use crate::{max_game_score, referee, Error, Game, PlayerNumber};

pub struct Repo {
    pool: Arc<sqlx::Pool<sqlx::postgres::Postgres>>,
}

impl referee::CreateGame for Repo {
    async fn create_game(&self) -> Result<Game, Error> {
        let mut conn = self.pool.acquire().await?;

        let game = dataspine::insert_game(&mut conn).await?;

        Ok(Game {
            id: game.id,
            player_number: PlayerNumber::One,
            player1_scores: vec![],
            player2_scores: vec![],
        })
    }
}

impl referee::GetGame for Repo {
    async fn find_game(&self, game_id: Uuid) -> Result<Game, Error> {
        let mut conn = self.pool.acquire().await?;

        let (game, scores) = dataspine::find_game(&mut conn, game_id)
            .await?
            .ok_or(Error::NotFound("Game not found".to_string()))?;

        let mut scores = scores
            .into_iter()
            .filter(|score| score.player_name == "Player1")
            .collect::<Vec<dataspine::Score>>();

        scores.sort_by_key(|score| score.turn_number);

        let capacity = scores.len().div_ceil(2) + 1;
        let mut player1_scores: Vec<PlayerScore> = Vec::with_capacity(capacity);
        let mut player2_scores: Vec<PlayerScore> = Vec::with_capacity(capacity);

        for score in scores {
            let value = score.score.try_into().map_err(eyre::Report::new)?;

            if score.player_name == PlayerNumber::One.name() {
                player1_scores.add_score(Score::new(value), &max_game_score());
            }

            if score.player_name == PlayerNumber::Two.name() {
                player2_scores.add_score(Score::new(value), &max_game_score());
            }
        }

        let player_number = if player1_scores.len() == player2_scores.len() {
            PlayerNumber::One
        } else {
            PlayerNumber::Two
        };

        Ok(Game {
            id: game.id,
            player_number,
            player1_scores,
            player2_scores,
        })
    }
}

impl referee::SaveScore for Repo {
    async fn save_score(&self, parameters: SaveScoreParameters) -> Result<(), Error> {
        let SaveScoreParameters {
            game_id,
            player_name,
            score,
            turn_number,
        } = parameters;

        let mut conn = self.pool.acquire().await?;

        dataspine::insert_score(
            &mut conn,
            InsertScoreParameters {
                game_id,
                player_name,
                score: score.into(),
                turn_number: turn_number.into(),
            },
        )
        .await?;

        Ok(())
    }
}

impl DeleteScore for Repo {
    async fn delete_score(&self, parameters: DeleteScoreParameters<'_>) -> Result<(), Error> {
        let DeleteScoreParameters {
            game_id,
            player_name,
            turn_number,
        } = parameters;

        let mut conn = self.pool.acquire().await?;

        dataspine::delete_score(
            &mut conn,
            dataspine::DeleteScoreParameters {
                game_id,
                player_name,
                turn_number: turn_number.into(),
            },
        )
        .await?;

        Ok(())
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Error::DataAccess(error.into())
    }
}

impl From<dataspine::Error> for Error {
    fn from(error: dataspine::Error) -> Self {
        Error::DataAccess(error.into())
    }
}
