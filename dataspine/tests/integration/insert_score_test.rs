use dataspine::Repo;
use playground::{
    referee::{InsertGamePreview, InsertScore, InsertScoreParameters},
    PlayerScore, Points, Score,
};
use sqlx::PgPool;

#[sqlx::test]
async fn it_inserts_score(pool: PgPool) -> anyhow::Result<()> {
    let repo = Repo::new(pool);
    let game_preview = repo.insert_game_preview().await?;

    let score_details = repo
        .insert_score(InsertScoreParameters {
            game_id: game_preview.game_id(),
            player_number: 1,
            player_score: PlayerScore::regular(Score::try_from(17)?),
            round_number: 1,
        })
        .await?;

    assert_eq!(score_details.player_number(), 1);

    match score_details.player_score() {
        PlayerScore::Regular(score) => assert_eq!(score.points(), Points::from(17)),
        _ => assert!(false),
    }

    Ok(())
}
