use crate::ScoreRow;
use sqlx::PgConnection;
use uuid::Uuid;
use playground as plg;

pub async fn list_scores(conn: &mut PgConnection, game_id: Uuid) -> Result<Vec<ScoreRow>, sqlx::Error> {
    let scores = sqlx::query_as!(
        ScoreRow,
        r#"
SELECT id, game_id, player_name, score, round_number, insert_time
FROM playground.scores
WHERE game_id = $1
        "#,
        game_id
    )
    .fetch_all(conn)
    .await?;

    Ok(scores)
}

pub fn playground_error(err: sqlx::Error) -> plg::Error {
    eyre::Report::from(err).into()
}
