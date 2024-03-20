use dataspine::Repo;
use playground::{
    referee::{InsertGamePreview, InsertScore, InsertScoreParameters},
    PlayerNumber, PlayerScore,
};
use sqlx::PgPool;

#[sqlx::test]
async fn it_inserts_score(pool: PgPool) -> anyhow::Result<()> {
    let repo = Repo::new(pool);
    let game_preview = repo.insert_game_preview().await?;

    let score_details = repo
        .insert_score(InsertScoreParameters {
            game_id: game_preview.game_id(),
            player_number: PlayerNumber::One,
            player_score: PlayerScore::score(17),
            round_number: 1,
        })
        .await?;

    assert_eq!(
        Into::<i32>::into(score_details.player_number()),
        Into::<i32>::into(PlayerNumber::One)
    );
    assert_eq!(score_details.player_score().into_inner(), 17);

    Ok(())
}
