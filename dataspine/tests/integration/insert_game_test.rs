use crate::helpers::count_games;
use dataspine::Repo;
use playground::referee::InsertGamePreview;
use sqlx::PgPool;

#[sqlx::test]
async fn it_inserts_game(pool: PgPool) -> anyhow::Result<()> {
    let was = count_games(&pool).await?;
    let _game = Repo::new(pool.clone()).insert_game_preview().await?;
    let now = count_games(&pool).await?;

    assert_eq!(now - was, 1);

    Ok(())
}
