use dataspine::Repo;
use playground::spectator::ListGamePreviews;
use sqlx::PgPool;

#[sqlx::test(fixtures("games"))]
async fn it_lists_games(pool: PgPool) -> anyhow::Result<()> {
    let game_previews = Repo::new(pool).list_game_previews().await?;

    assert_eq!(game_previews.len(), 1);

    Ok(())
}
