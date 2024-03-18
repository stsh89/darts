use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Game {
    id: Uuid,
    start_time: DateTime<Utc>,
}

pub struct LoadGameParameters {
    pub id: Uuid,
    pub start_time: DateTime<Utc>,
}

impl Game {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn load(parameters: LoadGameParameters) -> Self {
        let LoadGameParameters { id, start_time } = parameters;

        Self { id, start_time }
    }

    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }
}
