use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn get_game_id(pool: &PgPool) -> anyhow::Result<Uuid> {
    let id = sqlx::query("SELECT id FROM playground.games LIMIT 1")
        .fetch_one(pool)
        .await?
        .get("id");

    Ok(id)
}

pub async fn get_score_id(pool: &PgPool) -> anyhow::Result<Uuid> {
    let id = sqlx::query(
        r#"
SELECT id
FROM playground.scores
ORDER BY turn_number DESC, player_number DESC LIMIT 1
        "#,
    )
    .fetch_one(pool)
    .await?
    .get("id");

    Ok(id)
}

pub async fn count_games(pool: &PgPool) -> anyhow::Result<i64> {
    let count = sqlx::query("SELECT COUNT(*) count FROM playground.games")
        .fetch_one(pool)
        .await?
        .get("count");

    Ok(count)
}

pub async fn count_scores(pool: &PgPool) -> anyhow::Result<i64> {
    let count = sqlx::query("SELECT COUNT(*) count FROM playground.scores")
        .fetch_one(pool)
        .await?
        .get("count");

    Ok(count)
}
