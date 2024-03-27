use crate::helpers;
use dataspine::Repo;
use playground::coordinator::{GetGame, UpdateGame};
use sqlx::PgPool;

#[sqlx::test(fixtures("games"))]
async fn it_updates_game(pool: PgPool) -> anyhow::Result<()> {
    let game_id = helpers::get_game_id(&pool).await?;

    let game = Repo::new(pool.clone()).get_game(game_id).await?;
    let result = Repo::new(pool).update_game(&game).await;

    assert!(result.is_ok());

    Ok(())
}
