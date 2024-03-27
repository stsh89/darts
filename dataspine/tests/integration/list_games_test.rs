use dataspine::Repo;
use playground::coordinator::ListGames;
use sqlx::PgPool;

#[sqlx::test(fixtures("games"))]
async fn it_lists_games(pool: PgPool) -> anyhow::Result<()> {
    let result = Repo::new(pool).list_games().await;

    assert!(result.is_ok());

    Ok(())
}
