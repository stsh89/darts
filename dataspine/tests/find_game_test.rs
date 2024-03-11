mod helpers;

use dataspine::find_game;
use helpers::get_game_id;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(fixtures("games"))]
async fn it_finds_game(pool: PgPool) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;

    let game_id = get_game_id(&mut conn).await?;

    let (game, scores) = find_game(&mut conn, game_id)
        .await?
        .ok_or(anyhow::anyhow!("Game not found"))?;

    assert_eq!(game.id, game_id);
    assert_eq!(scores.len(), 4);

    Ok(())
}

#[sqlx::test(fixtures("games"))]
async fn it_does_not_find_game(pool: PgPool) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;

    let result = find_game(&mut conn, Uuid::nil()).await?;

    assert!(result.is_none());

    Ok(())
}
