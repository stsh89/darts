use crate::helpers;
use dataspine::Repo;
use playground::referee::GetGame;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(fixtures("games"))]
async fn it_gets_game(pool: PgPool) -> anyhow::Result<()> {
    let game_id = helpers::get_game_id(&pool).await?;

    let result = Repo::new(pool).get_game(game_id).await;

    assert!(result.is_ok());

    Ok(())
}

#[sqlx::test(fixtures("games"))]
async fn it_does_not_get_game_state(pool: PgPool) -> anyhow::Result<()> {
    let result = Repo::new(pool).get_game(Uuid::nil()).await;

    assert!(result.is_err());

    Ok(())
}
