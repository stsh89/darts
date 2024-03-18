use crate::{
    games::{FindGame, InsertGame, ListGames},
    scores::{DeleteScore, InserScore, ListScores},
    GameRow, InsertScoreParameters, ScoreRow,
};
use playground::{
    referee, spectator, Error, Game, GameState, LoadGameParameters, LoadGameStateParameters,
    LoadScoreDetailsParameters, PlayerScore, Score, ScoreDetails,
};
use sqlx::{pool::PoolConnection, Postgres};
use uuid::Uuid;

const POINTS_KIND_SCORE: &str = "score";
const POINTS_KIND_OVERTHROW: &str = "overthrow";

struct Points {
    number: i32,
    kind: String,
}

pub struct Repo {
    pool: sqlx::Pool<sqlx::postgres::Postgres>,
}

impl playground::GetGameState for Repo {
    async fn get_game_state(&self, game_id: Uuid) -> Result<GameState, Error> {
        let game = self
            .conn()
            .await?
            .find_game(game_id)
            .await
            .map_err(|err| Error::Repo(err.into()))?
            .ok_or(Error::NotFound("Game not found".to_string()))?
            .into();

        let score_details = self
            .conn()
            .await?
            .list_scores(game_id)
            .await
            .map_err(|err| Error::Repo(err.into()))?
            .into_iter()
            .map(TryFrom::try_from)
            .collect::<Result<Vec<ScoreDetails>, Error>>()?;

        let game_state = GameState::load(LoadGameStateParameters {
            game,
            score_details,
        })?;

        Ok(game_state)
    }
}

impl referee::DeleteScore for Repo {
    async fn delete_score(&self, id: Uuid) -> Result<(), Error> {
        self.conn()
            .await?
            .delete_score(id)
            .await
            .map_err(|err| Error::Repo(err.into()))?;

        Ok(())
    }
}

impl referee::InsertGame for Repo {
    async fn insert_game(&self) -> Result<Game, Error> {
        let game = self
            .conn()
            .await?
            .insert_game()
            .await
            .map_err(|err| Error::Repo(err.into()))?
            .into();

        Ok(game)
    }
}

impl referee::InsertScore for Repo {
    async fn insert_score(
        &self,
        parameters: referee::InsertScoreParameters,
    ) -> Result<ScoreDetails, Error> {
        let referee::InsertScoreParameters {
            game_id,
            player_number,
            player_score,
            round_number: turn_number,
        } = parameters;

        let points: Points = player_score.into();

        let score_details = self
            .conn()
            .await?
            .insert_score(InsertScoreParameters {
                game_id,
                player_number: player_number.into(),
                points_kind: points.kind,
                points_number: points.number,
                turn_number: turn_number.into(),
            })
            .await
            .map_err(|err| Error::Repo(err.into()))?
            .try_into()?;

        Ok(score_details)
    }
}

impl spectator::ListGames for Repo {
    async fn list_games(&self) -> Result<Vec<Game>, Error> {
        let games = self
            .conn()
            .await?
            .list_games()
            .await
            .map_err(|err| Error::Repo(err.into()))?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(games)
    }
}

// impl plg::spectator::ListGames for Repo {
//     async fn list_games(&self) -> Result<Vec<plg::Game>, plg::Error> {
//         self.list_games().await
//     }
// }

// impl plg::referee::CreateGame for Repo {
//     async fn create_game(&self) -> Result<plg::Game, plg::Error> {
//         self.create_game().await
//     }
// }

// impl plg::GetGame for Repo {
//     async fn get_game(&self, game_id: Uuid) -> Result<plg::GameMain, plg::Error> {
//         use plg::score_tracker::AddScore;

//         let mut conn = self.conn().await?;

//         let (game, scores) = find_game(&mut conn, game_id)
//             .await
//             .map_err(playground_error)?
//             .ok_or(plg::Error::NotFound("Game not found".to_string()))?;

//         let mut scores = scores.into_iter().collect::<Vec<ScoreRow>>();

//         scores.sort_by_key(|score| score.turn_number);

//         let capacity = scores.len().div_ceil(2) + 1;
//         let mut player1_scores: Vec<plg::PlayerScore> = Vec::with_capacity(capacity);
//         let mut player2_scores: Vec<plg::PlayerScore> = Vec::with_capacity(capacity);

//         for score in scores {
//             let value = score.points_number.try_into().map_err(|_err| {
//                 plg::Error::UnexpectedError(eyre::eyre!("Failed to convert score"))
//             })?;

//             if score.player_name == plg::PlayerNumber::One.name() {
//                 player1_scores.add_score(plg::Score::new(value), &plg::max_game_score());
//             }

//             if score.player_name == plg::PlayerNumber::Two.name() {
//                 player2_scores.add_score(plg::Score::new(value), &plg::max_game_score());
//             }
//         }

//         let player_number = if player1_scores.len() == player2_scores.len() {
//             plg::PlayerNumber::One
//         } else {
//             plg::PlayerNumber::Two
//         };

//         Ok(plg::GameMain {
//             id: game.id,
//             player_number,
//             player1_scores,
//             player2_scores,
//             start_time: game.insert_time,
//         })
//     }
// }

// impl plg::referee::SaveScore for Repo {
//     async fn save_score(
//         &self,
//         parameters: plg::referee::SaveScoreParameters,
//     ) -> Result<(), plg::Error> {
//         let plg::referee::SaveScoreParameters {
//             game_id,
//             player_name,
//             score,
//             turn_number,
//         } = parameters;

//         let mut conn = self.conn().await?;

//         insert_score(
//             &mut conn,
//             InsertScoreParameters {
//                 game_id,
//                 player_name,
//                 score: score.into(),
//                 turn_number: turn_number.into(),
//             },
//         )
//         .await
//         .map_err(playground_error)?;

//         Ok(())
//     }
// }

// impl plg::referee::DeleteScore for Repo {
//     async fn delete_score(
//         &self,
//         parameters: plg::referee::DeleteScoreParameters<'_>,
//     ) -> Result<(), plg::Error> {
//         let plg::referee::DeleteScoreParameters {
//             game_id,
//             player_name,
//             turn_number,
//         } = parameters;

//         let mut conn = self.conn().await?;

//         delete_score(
//             &mut conn,
//             DeleteScoreParameters {
//                 game_id,
//                 player_name,
//                 turn_number: turn_number.into(),
//             },
//         )
//         .await
//         .map_err(playground_error)?;

//         Ok(())
//     }
// }

impl Repo {
    pub fn new(pool: sqlx::Pool<sqlx::postgres::Postgres>) -> Self {
        Self { pool }
    }

    async fn conn(&self) -> Result<PoolConnection<Postgres>, Error> {
        let conn = self
            .pool
            .acquire()
            .await
            .map_err(|err| Error::Repo(err.into()))?;

        Ok(conn)
    }
}

impl From<GameRow> for Game {
    fn from(value: GameRow) -> Self {
        let GameRow { id, insert_time } = value;

        Self::load(LoadGameParameters {
            id,
            start_time: insert_time,
        })
    }
}

impl TryFrom<ScoreRow> for ScoreDetails {
    type Error = Error;

    fn try_from(value: ScoreRow) -> Result<Self, Error> {
        let ScoreRow {
            id,
            game_id,
            player_number,
            points_number,
            points_kind,
            turn_number,
            insert_time: _,
        } = value;

        let score = Points {
            kind: points_kind,
            number: points_number,
        }
        .try_into()?;

        Ok(Self::load(LoadScoreDetailsParameters {
            id,
            game_id,
            player_number: player_number.try_into()?,
            player_score: score,
            turn_number: turn_number.try_into().map_err(Into::<eyre::Report>::into)?,
        }))
    }
}

impl From<PlayerScore> for Points {
    fn from(value: PlayerScore) -> Self {
        match value {
            PlayerScore::Score(score) => Self {
                kind: POINTS_KIND_SCORE.to_string(),
                number: score.points().into(),
            },
            PlayerScore::Overthrow(score) => Self {
                kind: POINTS_KIND_OVERTHROW.to_string(),
                number: score.points().into(),
            },
        }
    }
}

impl TryFrom<Points> for PlayerScore {
    type Error = Error;

    fn try_from(value: Points) -> Result<Self, Error> {
        let Points { kind, number } = value;
        let points_number = number.try_into().map_err(Into::<eyre::Report>::into)?;

        if kind == POINTS_KIND_SCORE {
            return Ok(PlayerScore::Score(Score::new(points_number)));
        }

        if kind == POINTS_KIND_OVERTHROW {
            return Ok(PlayerScore::Overthrow(Score::new(points_number)));
        }

        Err(Error::Unexpected(eyre::eyre!("Invalid points kind")))
    }
}
