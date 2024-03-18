use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    games::ListGames, scores::ListScores, DeleteScore, EmptyResult, FindGame, GameRow, InserScore,
    InsertGame, InsertScoreParameters, MaybeRowResult, RowResult, RowsResult, ScoreRow,
};

impl DeleteScore for PgConnection {
    async fn delete_score(&mut self, id: Uuid) -> EmptyResult {
        delete_score(self, id).await
    }
}

impl FindGame for PgConnection {
    async fn find_game(&mut self, id: Uuid) -> MaybeRowResult<GameRow> {
        find_game(self, id).await
    }
}

impl InsertGame for PgConnection {
    async fn insert_game(&mut self) -> RowResult<GameRow> {
        insert_game(self).await
    }
}

impl InserScore for PgConnection {
    async fn insert_score(&mut self, parameters: InsertScoreParameters) -> RowResult<ScoreRow> {
        insert_score(self, parameters).await
    }
}

impl ListGames for PgConnection {
    async fn list_games(&mut self) -> RowsResult<GameRow> {
        list_games(self).await
    }
}

impl ListScores for PgConnection {
    async fn list_scores(&mut self, game_id: Uuid) -> RowsResult<ScoreRow> {
        list_scores(self, game_id).await
    }
}

async fn delete_score(conn: &mut PgConnection, id: Uuid) -> EmptyResult {
    sqlx::query!(r#"DELETE FROM playground.scores WHERE id = $1"#, id)
        .execute(conn)
        .await?;

    Ok(())
}

async fn find_game(conn: &mut PgConnection, id: Uuid) -> MaybeRowResult<GameRow> {
    let row = sqlx::query_as!(
        GameRow,
        r#"SELECT id, insert_time FROM playground.games WHERE id = $1"#,
        id,
    )
    .fetch_optional(conn.as_mut())
    .await?;

    Ok(row)
}

async fn insert_game(conn: &mut PgConnection) -> RowResult<GameRow> {
    let row = sqlx::query_as!(
        GameRow,
        r#"INSERT INTO playground.games DEFAULT VALUES RETURNING id, insert_time"#
    )
    .fetch_one(conn)
    .await?;

    Ok(row)
}

async fn insert_score(
    conn: &mut PgConnection,
    parameters: InsertScoreParameters,
) -> RowResult<ScoreRow> {
    let InsertScoreParameters {
        game_id,
        player_number,
        points_kind,
        points_number,
        turn_number,
    } = parameters;

    let row = sqlx::query_as!(
        ScoreRow,
        r#"
INSERT INTO playground.scores (
    game_id,
    player_number,
    points_kind,
    points_number,
    turn_number
) VALUES (
    $1, $2, $3, $4, $5
) RETURNING id, game_id, player_number, points_kind, points_number, turn_number, insert_time
        "#,
        game_id,
        player_number,
        points_kind,
        points_number,
        turn_number
    )
    .fetch_one(conn)
    .await?;

    Ok(row)
}

async fn list_games(conn: &mut PgConnection) -> RowsResult<GameRow> {
    let rows = sqlx::query_as!(
        GameRow,
        r#"SELECT id, insert_time FROM playground.games ORDER BY insert_time DESC LIMIT 10"#
    )
    .fetch_all(conn)
    .await?;

    Ok(rows)
}

async fn list_scores(conn: &mut PgConnection, game_id: Uuid) -> RowsResult<ScoreRow> {
    let rows = sqlx::query_as!(
        ScoreRow,
        r#"
SELECT id, game_id, player_number, points_kind, points_number, turn_number, insert_time
FROM playground.scores
WHERE game_id = $1
ORDER BY turn_number, player_number"#,
        game_id
    )
    .fetch_all(conn)
    .await?;

    Ok(rows)
}
