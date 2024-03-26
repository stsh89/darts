use crate::helpers::count_games;
use dataspine::Repo;
use playground::{referee::SaveGame, Game, NewGameParameters, Number, Points};
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn it_saves_game(pool: PgPool) -> anyhow::Result<()> {
    let was = count_games(&pool).await?;
    let mut game = Game::new(NewGameParameters {
        points_limit: Points::new(301),
        players_number: Number::one(),
    })?;

    Repo::new(pool.clone()).save_game(&mut game).await?;
    let now = count_games(&pool).await?;

    assert_eq!(now - was, 1);
    assert!(game.id().is_some());
    assert!(game.start_time().is_some());

    Ok(())
}

#[sqlx::test]
async fn it_does_not_save_persisted_game(pool: PgPool) -> anyhow::Result<()> {
    let was = count_games(&pool).await?;
    let mut game = Game::new(NewGameParameters {
        points_limit: Points::new(301),
        players_number: Number::one(),
    })?;
    game.assign_id(Uuid::nil());

    let result = Repo::new(pool.clone()).save_game(&mut game).await;
    let now = count_games(&pool).await?;

    assert_eq!(now - was, 0);
    assert!(result.is_err());

    Ok(())
}
