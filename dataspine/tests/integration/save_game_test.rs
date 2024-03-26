use crate::helpers::{count_games, count_scores};
use dataspine::Repo;
use playground::{referee::SaveGame, Game, NewGameParameters, Number, Points, Score};
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
async fn it_saves_game_and_rounds(pool: PgPool) -> anyhow::Result<()> {
    let games_count_was = count_games(&pool).await?;
    let scores_count_was = count_scores(&pool).await?;
    let mut game = Game::new(NewGameParameters {
        points_limit: Points::new(301),
        players_number: Number::one(),
    })?;
    game.count_score(Score::new(7)?)?;
    game.count_score(Score::new(23)?)?;

    Repo::new(pool.clone()).save_game(&mut game).await?;
    let games_count_now = count_games(&pool).await?;
    let scores_count_now = count_scores(&pool).await?;

    assert_eq!(games_count_now - games_count_was, 1);
    assert_eq!(scores_count_now - scores_count_was, 2);
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

    Repo::new(pool.clone()).save_game(&mut game).await?;
    let now = count_games(&pool).await?;

    assert_eq!(now - was, 0);
    assert!(game.id().unwrap().is_nil());

    Ok(())
}
