use chrono::{DateTime, Utc};
use sqlx::PgConnection;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unexpected number of affected rows: {0}")]
    UnexpectedNumberOfAffectedRows(u64),

    #[error("Sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

pub struct Game {
    pub id: Uuid,
    pub insert_time: DateTime<Utc>,
}

pub struct Score {
    pub id: Uuid,
    pub game_id: Uuid,
    pub player_name: String,
    pub score: i32,
    pub turn_number: i32,
    pub insert_time: DateTime<Utc>,
}

pub struct InsertScoreParameters {
    pub game_id: Uuid,
    pub player_name: String,
    pub score: i32,
    pub turn_number: i32,
}

pub struct DeleteScoreParameters<'a> {
    pub game_id: Uuid,
    pub player_name: &'a str,
    pub turn_number: i32,
}

pub async fn insert_game(conn: &mut PgConnection) -> Result<Game, Error> {
    let game = sqlx::query_as!(
        Game,
        r#"INSERT INTO playground.games DEFAULT VALUES RETURNING id, insert_time"#
    )
    .fetch_one(conn)
    .await?;

    Ok(game)
}

pub async fn find_game(
    conn: &mut PgConnection,
    id: Uuid,
) -> Result<Option<(Game, Vec<Score>)>, Error> {
    let game = sqlx::query_as!(
        Game,
        r#"SELECT id, insert_time FROM playground.games WHERE id = $1"#,
        id,
    )
    .fetch_optional(conn.as_mut())
    .await?;

    let Some(game) = game else {
        return Ok(None);
    };

    let scores = list_scores(conn, id).await?;

    Ok(Some((game, scores)))
}

pub async fn insert_score(
    conn: &mut PgConnection,
    parameters: InsertScoreParameters,
) -> Result<Score, Error> {
    let InsertScoreParameters {
        game_id,
        player_name,
        score,
        turn_number,
    } = parameters;

    let score = sqlx::query_as!(
        Score,
        r#"
INSERT INTO playground.scores (
    game_id,
    player_name,
    score,
    turn_number
) VALUES (
    $1, $2, $3, $4
) RETURNING id, game_id, player_name, score, turn_number, insert_time
        "#,
        game_id,
        player_name,
        score,
        turn_number
    )
    .fetch_one(conn)
    .await?;

    Ok(score)
}

pub async fn delete_score(
    conn: &mut PgConnection,
    parameters: DeleteScoreParameters<'_>,
) -> Result<(), Error> {
    let DeleteScoreParameters {
        game_id,
        player_name,
        turn_number,
    } = parameters;

    let result = sqlx::query!(
        r#"
DELETE FROM playground.scores
WHERE game_id = $1
AND player_name = $2
AND turn_number = $3
        "#,
        game_id,
        player_name,
        turn_number
    )
    .execute(conn)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::Sqlx(sqlx::Error::RowNotFound));
    }

    if result.rows_affected() > 1 {
        return Err(Error::UnexpectedNumberOfAffectedRows(
            result.rows_affected(),
        ));
    }

    Ok(())
}

async fn list_scores(conn: &mut PgConnection, game_id: Uuid) -> Result<Vec<Score>, Error> {
    let scores = sqlx::query_as!(
        Score,
        r#"
SELECT id, game_id, player_name, score, turn_number, insert_time
FROM playground.scores
WHERE game_id = $1
        "#,
        game_id
    )
    .fetch_all(conn)
    .await?;

    Ok(scores)
}
