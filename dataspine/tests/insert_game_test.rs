use dataspine::insert_game;
use sqlx::PgPool;

#[sqlx::test]
async fn it_inserts_game(pool: PgPool) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;

    let game = insert_game(&mut conn).await?;

    assert!(!game.id.is_nil());

    Ok(())
}
