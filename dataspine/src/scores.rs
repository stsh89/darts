use crate::{EmptyResult, RowResult, RowsResult};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait DeleteScore {
    async fn delete_score(&mut self, id: Uuid) -> EmptyResult;
}

pub trait InserScore {
    async fn insert_score(&mut self, parameters: InsertScoreParameters) -> RowResult<ScoreRow>;
}

pub trait ListScores {
    async fn list_scores(&mut self, game_id: Uuid) -> RowsResult<ScoreRow>;
}

pub struct InsertScoreParameters {
    pub game_id: Uuid,
    pub player_number: i32,
    pub points_kind: String,
    pub points_number: i32,
    pub round_number: i32,
}

pub struct ScoreRow {
    pub id: Uuid,
    pub game_id: Uuid,
    pub player_number: i32,
    pub points_number: i32,
    pub points_kind: String,
    pub round_number: i32,
    pub insert_time: DateTime<Utc>,
}
