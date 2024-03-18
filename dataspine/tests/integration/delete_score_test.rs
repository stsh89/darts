use crate::helpers;
use dataspine::Repo;
use playground::referee::DeleteScore;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(fixtures("games"))]
async fn it_deletes_score(pool: PgPool) -> anyhow::Result<()> {
    let was = helpers::count_scores(&pool).await?;
    let score_id = helpers::get_score_id(&pool).await?;
    let result = Repo::new(pool.clone()).delete_score(score_id).await;
    let now = helpers::count_scores(&pool).await?;

    assert!(result.is_ok());
    assert_eq!(was - now, 1);

    Ok(())
}

#[sqlx::test(fixtures("games"))]
async fn it_does_not_delete_score(pool: PgPool) -> anyhow::Result<()> {
    let was = helpers::count_scores(&pool).await?;
    let result = Repo::new(pool.clone()).delete_score(Uuid::nil()).await;
    let now = helpers::count_scores(&pool).await?;

    assert!(result.is_ok());
    assert_eq!(was, now);

    Ok(())
}
