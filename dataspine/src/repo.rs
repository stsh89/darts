use crate::{
    games::{FindGame, InsertGame, ListGames},
    scores::ListScores,
    GameRow, ScoreRow,
};
use playground::{
    referee, spectator, Error, Game, GamePreview, LoadGameParameters, LoadGamePreviewParameters,
    LoadRoundParameters, Number, PlayerScore, Round, Score,
};
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, PgPool, Postgres};
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

impl spectator::GetGame for Repo {
    async fn get_game(&self, id: Uuid) -> Result<Game, Error> {
        get_game(self, id).await
    }
}

impl referee::GetGame for Repo {
    async fn get_game(&self, id: Uuid) -> Result<Game, Error> {
        get_game(self, id).await
    }
}

impl referee::SaveGame for Repo {
    async fn save_game(&self, game: &mut Game) -> Result<(), Error> {
        if game.is_persisted() {
            return Err(Error::Unexpected(eyre::eyre!("Game is already persisted")));
        }

        let game_row = self
            .conn()
            .await?
            .insert_game()
            .await
            .map_err(|err| Error::Repo(err.into()))?;

        game.set_id(game_row.id);
        game.set_start_time(game_row.insert_time);

        Ok(())
    }
}

impl spectator::ListGamePreviews for Repo {
    async fn list_game_previews(&self) -> Result<Vec<GamePreview>, Error> {
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

impl Repo {
    async fn conn(&self) -> Result<PoolConnection<Postgres>, Error> {
        let conn = self
            .pool
            .acquire()
            .await
            .map_err(|err| Error::Repo(err.into()))?;

        Ok(conn)
    }

    pub async fn from_database_url(database_url: &str) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .connect(database_url)
            .await
            .map_err(|err| Error::Repo(err.into()))?;

        Ok(Self { pool })
    }

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl From<GameRow> for GamePreview {
    fn from(value: GameRow) -> Self {
        let GameRow { id, insert_time } = value;

        Self::load(LoadGamePreviewParameters {
            game_id: id,
            start_time: insert_time,
        })
    }
}

impl TryFrom<ScoreRow> for Round {
    type Error = Error;

    fn try_from(value: ScoreRow) -> Result<Self, Error> {
        let ScoreRow {
            id,
            game_id: _,
            player_number,
            points_number,
            points_kind,
            round_number,
            insert_time: _,
        } = value;

        let player_score = Points {
            kind: points_kind,
            number: points_number,
        }
        .try_into()?;

        let player_number = Number::new(
            player_number
                .try_into()
                .map_err(Into::<eyre::Report>::into)?,
        )?;
        let number = Number::new(
            round_number
                .try_into()
                .map_err(Into::<eyre::Report>::into)?,
        )?;

        Self::load(LoadRoundParameters {
            id,
            player_number,
            player_score,
            number,
        })
    }
}

impl From<PlayerScore> for Points {
    fn from(value: PlayerScore) -> Self {
        match value {
            PlayerScore::Regular(score) => Self {
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

        if kind == POINTS_KIND_SCORE {
            return Ok(PlayerScore::Regular(Score::try_from(number)?));
        }

        if kind == POINTS_KIND_OVERTHROW {
            return Ok(PlayerScore::Overthrow(Score::try_from(number)?));
        }

        Err(Error::Unexpected(eyre::eyre!("Invalid points kind")))
    }
}

async fn get_game(repo: &Repo, id: Uuid) -> Result<Game, Error> {
    let game_row: GameRow = repo
        .conn()
        .await?
        .find_game(id)
        .await
        .map_err(|err| Error::Repo(err.into()))?
        .ok_or(Error::NotFound("Game not found".to_string()))?;

    let score_details = repo
        .conn()
        .await?
        .list_scores(id)
        .await
        .map_err(|err| Error::Repo(err.into()))?
        .into_iter()
        .map(TryFrom::try_from)
        .collect::<Result<Vec<Round>, Error>>()?;

    let game = Game::load(LoadGameParameters {
        id,
        rounds: score_details,
        start_time: game_row.insert_time,
    })?;

    Ok(game)
}
