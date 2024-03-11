mod helpers;

use dataspine::{insert_game, insert_score, InsertScoreParameters};
use sqlx::PgPool;

#[sqlx::test]
async fn it_inserts_score(pool: PgPool) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;
    let game = insert_game(&mut conn).await?;

    let score = insert_score(
        &mut conn,
        InsertScoreParameters {
            game_id: game.id,
            player_name: "Player1".to_string(),
            score: 17,
            turn_number: 1,
        },
    )
    .await?;

    assert_eq!(score.player_name, "Player1");
    assert_eq!(score.score, 17);
    assert_eq!(score.game_id, game.id);

    Ok(())
}
