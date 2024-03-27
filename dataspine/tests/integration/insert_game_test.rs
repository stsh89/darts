use crate::helpers;
use dataspine::Repo;
use playground::{coordinator::InsertGame, Game, NewGameParameters, Number, Points};
use sqlx::PgPool;

#[sqlx::test]
async fn it_saves_game(pool: PgPool) -> anyhow::Result<()> {
    let count_games_was = helpers::count_games(&pool).await?;
    let game = Game::new(NewGameParameters {
        points_limit: Points::new(301),
        players_number: Number::one(),
    })?;

    let id = Repo::new(pool.clone()).insert_game(&game).await?;
    let count_games_now = helpers::count_games(&pool).await?;

    assert!(!id.is_nil());
    assert_eq!(count_games_now - count_games_was, 1);

    Ok(())
}
