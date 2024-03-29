use dataspine::Repo;
use playground::coordinator::ListGames;
use sqlx::PgPool;

#[sqlx::test(fixtures("games"))]
async fn it_lists_games(pool: PgPool) -> anyhow::Result<()> {
    let games = Repo::new(pool).list_games().await?;

    assert_eq!(games.len(), 1);

    Ok(())
}
