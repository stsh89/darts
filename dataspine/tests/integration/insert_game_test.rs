use crate::helpers;
use dataspine::Repo;
use playground::{coordinator::InsertGame, Game, NewGameParameters, Number, Points};
use sqlx::PgPool;

#[sqlx::test]
async fn it_saves_game(pool: PgPool) -> anyhow::Result<()> {
    let count_games_was = helpers::count_games(&pool).await?;
    let mut game = Game::new(NewGameParameters {
        points_limit: Points::new(301),
        players_number: Number::one(),
    })?;

    Repo::new(pool.clone()).insert_game(&mut game).await?;
    let count_games_now = helpers::count_games(&pool).await?;

    assert!(game.id().is_some());
    assert!(game.create_time().is_some());
    assert!(game.update_time().is_some());

    assert_eq!(count_games_now - count_games_was, 1);

    Ok(())
}
