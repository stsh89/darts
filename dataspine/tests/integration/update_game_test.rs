use crate::helpers;
use dataspine::Repo;
use playground::coordinator::{GetGame, UpdateGame};
use sqlx::PgPool;

#[sqlx::test(fixtures("games"))]
async fn it_updates_game(pool: PgPool) -> anyhow::Result<()> {
    let game_id = helpers::get_game_id(&pool).await?;

    let mut game = Repo::new(pool.clone()).get_game(game_id).await?;
    let update_time_was = game.update_time();

    Repo::new(pool).update_game(&mut game).await?;
    let update_time_now = game.update_time();

    assert!(update_time_was < update_time_now);

    Ok(())
}
