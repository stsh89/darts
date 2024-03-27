use crate::{FindGame, GameRow, InsertGame, ListGames, NewGameRow, RoundsColumn, UpdateGame};
use playground::{Error, Game};
use sqlx::{types::Json, PgConnection};
use uuid::Uuid;

impl FindGame for PgConnection {
    async fn find_game(&mut self, id: Uuid) -> Result<Option<GameRow>, Error> {
        let row = sqlx::query_as!(
            GameRow,
            r#"
SELECT
    id,
    players_number,
    points_limit,
    rounds as "rounds!: Json<Vec<RoundsColumn>>",
    start_time,
    end_time
FROM playground.games
WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(self)
        .await
        .map_err(eyre::Report::new)?;

        Ok(row)
    }
}

impl InsertGame for PgConnection {
    async fn insert_game(&mut self, game: &Game) -> Result<Uuid, Error> {
        let row = NewGameRow::from(game);

        let id = sqlx::query_scalar!(
            r#"
INSERT INTO playground.games
    (players_number, points_limit, rounds, start_time, end_time)
VALUES ($1, $2, $3, $4, $5)
RETURNING id"#,
            row.players_number,
            row.points_limit,
            row.rounds as _,
            row.start_time,
            row.end_time
        )
        .fetch_one(self)
        .await
        .map_err(eyre::Report::new)?;

        Ok(id)
    }
}

impl ListGames for PgConnection {
    async fn list_games(&mut self) -> Result<Vec<GameRow>, Error> {
        let rows = sqlx::query_as!(
            GameRow,
            r#"
        SELECT
            id,
            players_number,
            points_limit,
            rounds as "rounds!: Json<Vec<RoundsColumn>>",
            start_time,
            end_time
        FROM playground.games
        ORDER BY insert_time DESC LIMIT 10
            "#
        )
        .fetch_all(self)
        .await
        .map_err(eyre::Report::new)?;

        Ok(rows)
    }
}

impl UpdateGame for PgConnection {
    async fn update_game(&mut self, game: &Game) -> Result<(), Error> {
        let row = GameRow::from(game);

        sqlx::query!(
            r#"
UPDATE playground.games
SET players_number = $2, points_limit = $3, rounds = $4, start_time = $5, end_time = $6, update_time = default
WHERE id = $1
"#,
            row.id,
            row.players_number,
            row.points_limit,
            row.rounds as _,
            row.start_time,
            row.end_time
        )
        .execute(self)
        .await
        .map_err(eyre::Report::new)?;

        Ok(())
    }
}
