use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct GamePreview {
    game_id: Uuid,
    start_time: DateTime<Utc>,
}

pub struct LoadGamePreviewParameters {
    pub game_id: Uuid,
    pub start_time: DateTime<Utc>,
}

impl GamePreview {
    pub fn load(parameters: LoadGamePreviewParameters) -> Self {
        let LoadGamePreviewParameters {
            game_id,
            start_time,
        } = parameters;

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
