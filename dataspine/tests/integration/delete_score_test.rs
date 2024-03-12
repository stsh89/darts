use crate::helpers;
use dataspine::{delete_score, DeleteScoreParameters};
use sqlx::PgPool;

#[sqlx::test(fixtures("games"))]
async fn it_deletes_score(pool: PgPool) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;

    let game_id = helpers::get_game_id(&mut conn).await?;

    let result = delete_score(
        &mut conn,
        DeleteScoreParameters {
            game_id,
            player_name: "Player1",
            turn_number: 1,
        },
    )
    .await;

    assert!(result.is_ok());

    Ok(())
}

#[sqlx::test(fixtures("games"))]
async fn it_does_not_delete_score(pool: PgPool) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;

    let game_id = helpers::get_game_id(&mut conn).await?;

    let result = delete_score(
        &mut conn,
        DeleteScoreParameters {
            game_id,
            player_name: "Player10",
            turn_number: 102,
        },
    )
    .await;

    assert!(result.is_err());

    Ok(())
}
