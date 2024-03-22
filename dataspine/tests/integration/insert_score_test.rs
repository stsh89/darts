use dataspine::Repo;
use playground::{
    referee::{InsertGamePreview, InsertScore, InsertScoreParameters},
    PlayerNumber, PlayerScore, Points, Score,
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
            player_score: PlayerScore::regular(Score::try_from(17)?),
            round_number: 1,
        })
        .await?;

    assert_eq!(
        Into::<i32>::into(score_details.player_number()),
        Into::<i32>::into(PlayerNumber::One)
    );

    match score_details.player_score() {
        PlayerScore::Regular(score) => assert_eq!(score.points(), Points::from(17)),
        _ => assert!(false),
    }

    Ok(())
}
