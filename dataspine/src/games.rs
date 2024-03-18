use crate::{MaybeRowResult, RowResult, RowsResult};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait FindGame {
    async fn find_game(&mut self, id: Uuid) -> MaybeRowResult<GameRow>;
}

pub trait InsertGame {
    async fn insert_game(&mut self) -> RowResult<GameRow>;
}

pub trait ListGames {
    async fn list_games(&mut self) -> RowsResult<GameRow>;
}

pub struct GameRow {
    pub id: Uuid,
    pub insert_time: DateTime<Utc>,
}
