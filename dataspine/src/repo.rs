use crate::{
    games::{FindGame, ListGames},
    scores::ListScores,
    GameRow, ScoreRow,
};
use playground::{
    referee, spectator, Error, Game, GamePreview, LoadGameParameters, LoadGamePreviewParameters,
    LoadRoundParameters, Number, PlayerScore, Round, Score,
};
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, PgPool, Postgres, Transaction};
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
        let mut transaction = self.transaction().await?;

        if !game.is_persisted() {
            let row = sqlx::query_as!(
                GameRow,
                r#"INSERT INTO playground.games DEFAULT VALUES RETURNING id, insert_time"#
            )
            .fetch_one(transaction.as_mut())
            .await
            .map_err(eyre::Report::new)?;

            game.assign_id(row.id);
            game.assign_start_time(row.insert_time);
        }

        let rounds: Vec<&Round> = game.rounds().iter().filter(|r| !r.is_persisted()).collect();

        if !rounds.is_empty() {
            let game_ids: Vec<Uuid> = std::iter::repeat(game.id().unwrap())
                .take(rounds.len())
                .collect();
            let player_numbers: Vec<i32> = rounds
                .iter()
                .map(|r| r.player_number().value() as i32)
                .collect();
            let round_numbers: Vec<i32> =
                rounds.iter().map(|r| r.number().value() as i32).collect();
            let points_kinds: Vec<&str> = rounds
                .iter()
                .map(|r| match r.player_score() {
                    PlayerScore::Regular(_) => POINTS_KIND_SCORE,
                    PlayerScore::Overthrow(_) => POINTS_KIND_OVERTHROW,
                })
                .collect();
            let points_numbers: Vec<i32> = rounds
                .iter()
                .map(|r| r.player_score().score().points().into())
                .collect();

            let _row = sqlx::query!(
                r#"
INSERT INTO playground.scores (
    game_id,
    player_number,
    points_kind,
    points_number,
    round_number
)
SELECT
    game_id,
    player_number,
    points_kind,
    points_number,
    round_number
FROM UNNEST(
    $1::uuid[],
    $2::int[],
    $3::text[],
    $4::int[],
    $5::int[]
) as values(
    game_id,
    player_number,
    points_kind,
    points_number,
    round_number
)
RETURNING id
"#,
                &game_ids,
                &player_numbers,
                points_kinds as Vec<_>,
                &points_numbers,
                &round_numbers
            )
            .fetch_all(transaction.as_mut())
            .await
            .map_err(eyre::Report::new)?;
        }

        transaction.commit().await.map_err(eyre::Report::new)?;

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
            .map_err(eyre::Report::new)?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(games)
    }
}

impl Repo {
    async fn conn(&self) -> Result<PoolConnection<Postgres>, Error> {
        let conn = self.pool.acquire().await.map_err(eyre::Report::new)?;

        Ok(conn)
    }

    async fn transaction(&self) -> Result<Transaction<Postgres>, Error> {
        let transaction = self.pool.begin().await.map_err(eyre::Report::new)?;

        Ok(transaction)
    }

    pub async fn from_database_url(database_url: &str) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .connect(database_url)
            .await
            .map_err(eyre::Report::new)?;

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

        let number = number.try_into().map_err(eyre::Report::new)?;

        if kind == POINTS_KIND_SCORE {
            return Ok(PlayerScore::Regular(Score::new(number)?));
        }

        if kind == POINTS_KIND_OVERTHROW {
            return Ok(PlayerScore::Overthrow(Score::new(number)?));
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
        .map_err(eyre::Report::new)?
        .ok_or(Error::NotFound("Game not found".to_string()))?;

    let score_details = repo
        .conn()
        .await?
        .list_scores(id)
        .await
        .map_err(eyre::Report::new)?
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
