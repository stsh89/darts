use chrono::{DateTime, Utc};
use playground::{Error, Game};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};
use uuid::Uuid;

pub trait InsertGame {
    async fn insert_game(&mut self, game: &Game) -> Result<Uuid, Error>;
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

#[derive(FromRow)]
pub struct GameRow {
    pub id: Uuid,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub points_limit: i32,
    pub players_number: i32,
    pub rounds: Json<Vec<RoundsColumn>>,
}

pub struct NewGameRow {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub points_limit: i32,
    pub players_number: i32,
    pub rounds: Json<Vec<RoundsColumn>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RoundsColumn {
    pub round_number: i32,
    pub player_number: i32,
    pub points_kind: String,
    pub points: i32,
}

// impl From<JsonValue> for RoundColumn {
//     fn from(json: JsonValue) -> Self {
//         let json = json.as_object().unwrap();
//         Self {
//             round_number: json["round_number"].as_i64().unwrap() as i32,
//             player_number: json["player_number"].as_i64().unwrap() as i32,
//             points_kind: json["points_kind"].as_str().unwrap().to_string(),
//             points: json["points"].as_i64().unwrap() as i32,
//         }
//     }
// }
