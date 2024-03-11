use sqlx::{PgConnection, Row};
use uuid::Uuid;

pub async fn get_game_id(conn: &mut PgConnection) -> anyhow::Result<Uuid> {
    let id = sqlx::query("SELECT id FROM playground.games LIMIT 1")
        .fetch_one(conn)
        .await?
        .get("id");

    Ok(id)
}
