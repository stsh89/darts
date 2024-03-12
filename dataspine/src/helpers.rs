use sqlx::PgConnection;
use uuid::Uuid;
use crate::{Error, Score};

pub async fn list_scores(conn: &mut PgConnection, game_id: Uuid) -> Result<Vec<Score>, Error> {
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
