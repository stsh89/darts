use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_game_id(pool: &PgPool) -> anyhow::Result<Uuid> {
    let id = sqlx::query_scalar("SELECT id FROM playground.games LIMIT 1")
        .fetch_one(pool)
        .await?;

    Ok(id)
}

pub async fn count_games(pool: &PgPool) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar("SELECT COUNT(*) count FROM playground.games")
        .fetch_one(pool)
        .await?;

    Ok(count)
}
