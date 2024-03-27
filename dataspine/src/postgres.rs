use crate::{FindGame, GameRow, InsertGame, ListGames, RoundsColumn, UpdateGame};
use playground::{Error, Game};
use sqlx::{types::Json, PgConnection};
use uuid::Uuid;

impl FindGame for PgConnection {
    async fn find_game(&mut self, id: Uuid) -> Result<Option<GameRow>, Error> {
        let row = sqlx::query_file_as!(GameRow, "queries/find_game.sql", id,)
            .fetch_optional(self)
            .await
            .map_err(eyre::Report::new)?;

        Ok(row)
    }
}

impl InsertGame for PgConnection {
    async fn insert_game(&mut self, game: &Game) -> Result<Uuid, Error> {
        let row = GameRow::from(game);

        let id = sqlx::query_file_scalar!(
            "queries/insert_game.sql",
            row.end_time,
            row.players_number,
            row.points_limit,
            row.rounds as _,
            row.start_time
        )
        .fetch_one(self)
        .await
        .map_err(eyre::Report::new)?;

        Ok(id)
    }
}

impl ListGames for PgConnection {
    async fn list_games(&mut self) -> Result<Vec<GameRow>, Error> {
        let rows = sqlx::query_file_as!(GameRow, "queries/list_games.sql",)
            .fetch_all(self)
            .await
            .map_err(eyre::Report::new)?;

        Ok(rows)
    }
}

impl UpdateGame for PgConnection {
    async fn update_game(&mut self, game: &Game) -> Result<(), Error> {
        let row = GameRow::from(game);

        sqlx::query_file!(
            "queries/update_game.sql",
            row.id,
            row.end_time,
            row.players_number,
            row.points_limit,
            row.rounds as _,
            row.start_time,
        )
        .execute(self)
        .await
        .map_err(eyre::Report::new)?;

        Ok(())
    }
}
