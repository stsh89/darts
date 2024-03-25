use dataspine::Repo;
use playground::{
    referee::{InsertScore, InsertScoreParameters},
    PlayerScore, Points, Score,
};
use sqlx::PgPool;

use crate::helpers;

#[sqlx::test(fixtures("games"))]
async fn it_inserts_score(pool: PgPool) -> anyhow::Result<()> {
    let was = helpers::count_scores(&pool).await?;
    let game_id = helpers::get_game_id(&pool).await?;
    let repo = Repo::new(pool.clone());

    let score_details = repo
        .insert_score(InsertScoreParameters {
            game_id,
            player_number: 1,
            player_score: PlayerScore::regular(Score::try_from(17)?),
            round_number: 1,
        })
        .await?;
    let now = helpers::count_scores(&pool).await?;

    assert_eq!(now - was, 1);
    assert_eq!(score_details.player_number().value(), 1);

    match score_details.player_score() {
        PlayerScore::Regular(score) => assert_eq!(score.points(), Points::from(17)),
        _ => assert!(false),
    }

    Ok(())
}
