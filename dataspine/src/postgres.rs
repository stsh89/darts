use crate::{FindGame, GameRow, InsertGame, ListGames, RoundsColumnItem, UpdateGame};
use chrono::{DateTime, Utc};
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
    async fn insert_game(&mut self, game: &mut Game) -> Result<(), Error> {
        struct InsertReturnValues {
            id: Uuid,
            insert_time: DateTime<Utc>,
            update_time: DateTime<Utc>,
        }

        let end_time = game.end_time();
        let players_number = game.players_number().value() as i32;
        let points_limit = game.points_limit().value() as i32;
        let rounds: Vec<RoundsColumnItem> = game.rounds().iter().map(Into::into).collect();
        let start_time = game.start_time();

        let values = sqlx::query_file_as!(
            InsertReturnValues,
            "queries/insert_game.sql",
            end_time,
            players_number,
            points_limit,
            Json(rounds) as _,
            start_time
        )
        .fetch_one(self)
        .await
        .map_err(eyre::Report::new)?;

        game.assign_id(values.id);
        game.assign_create_time(values.insert_time);
        game.assign_update_time(values.update_time);

        Ok(())
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
        let id = game
            .id()
            .ok_or(eyre::eyre!("Trying to update game without id"))?;
        let end_time = game.end_time();
        let players_number = game.players_number().value() as i32;
        let points_limit = game.points_limit().value() as i32;
        let rounds: Vec<RoundsColumnItem> = game.rounds().iter().map(Into::into).collect();
        let start_time = game.start_time();

        sqlx::query_file!(
            "queries/update_game.sql",
            id,
            end_time,
            players_number,
            points_limit,
            Json(rounds) as _,
            start_time,
        )
        .execute(self)
        .await
        .map_err(eyre::Report::new)?;

        Ok(())
    }
}
