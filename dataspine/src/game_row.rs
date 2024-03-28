use chrono::{DateTime, Utc};
use playground::{Error, Game};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

pub trait InsertGame {
    async fn insert_game(&mut self, game: &mut Game) -> Result<(), Error>;
}

pub trait FindGame {
    async fn find_game(&mut self, id: Uuid) -> Result<Option<GameRow>, Error>;
}

pub trait ListGames {
    async fn list_games(&mut self) -> Result<Vec<GameRow>, Error>;
}

pub trait UpdateGame {
    async fn update_game(&mut self, game: &Game) -> Result<(), Error>;
}

pub struct GameRow {
    pub end_time: Option<DateTime<Utc>>,
    pub id: Uuid,
    pub insert_time: DateTime<Utc>,
    pub players_number: i32,
    pub points_limit: i32,
    pub rounds: Json<Vec<RoundsColumnItem>>,
    pub start_time: Option<DateTime<Utc>>,
    pub update_time: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RoundsColumnItem {
    pub round_number: i32,
    pub player_number: i32,
    pub points_kind: String,
    pub points: i32,
}
