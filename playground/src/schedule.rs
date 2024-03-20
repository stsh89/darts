use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct GamePreview {
    game_id: Uuid,
    start_time: DateTime<Utc>,
}

pub struct Schedule {
    game_previews: Vec<GamePreview>,
}

impl GamePreview {
    pub fn new(game_id: Uuid, start_time: DateTime<Utc>) -> Self {
        Self {
            game_id,
            start_time,
        }
    }

    pub fn game_id(&self) -> Uuid {
        self.game_id
    }

    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }
}

impl Schedule {
    pub fn game_previews(&self) -> &[GamePreview] {
        self.game_previews.as_slice()
    }

    pub fn into_game_previews(self) -> Vec<GamePreview> {
        self.game_previews
    }

    pub fn new(game_previews: Vec<GamePreview>) -> Self {
        Self { game_previews }
    }
}
