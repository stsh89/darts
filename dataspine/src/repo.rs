use crate::{
    games::{FindGame, InsertGame, ListGames, RoundsColumn, UpdateGame},
    GameRow, NewGameRow,
};
use playground::{
    coordinator, Error, Game, LoadGameParameters, NewRoundParameters, Number, PlayerScore, Points,
    Round, Score,
};
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, types::Json, PgPool, Postgres};
use uuid::Uuid;

const POINTS_KIND_REGULAR: &str = "regular";
const POINTS_KIND_OVERTHROW: &str = "overthrow";

pub struct Repo {
    pool: sqlx::Pool<sqlx::postgres::Postgres>,
}

impl coordinator::GetGame for Repo {
    async fn get_game(&self, id: Uuid) -> Result<Game, Error> {
        self.conn()
            .await?
            .find_game(id)
            .await?
            .ok_or(Error::NotFound(format!("Game {id}")))
            .map(TryInto::try_into)?
    }
}

impl coordinator::SaveGame for Repo {
    async fn save_game(&self, game: &Game) -> Result<Uuid, Error> {
        let mut conn = self.conn().await?;

        if let Some(id) = game.id() {
            conn.update_game(game).await?;
            return Ok(id);
        }

        conn.insert_game(game).await
    }
}

impl coordinator::ListGames for Repo {
    async fn list_games(&self) -> Result<Vec<Game>, Error> {
        let games = self
            .conn()
            .await?
            .list_games()
            .await
            .map_err(eyre::Report::new)?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<Game>, Error>>()?;

        Ok(games)
    }
}

impl Repo {
    async fn conn(&self) -> Result<PoolConnection<Postgres>, Error> {
        let conn = self.pool.acquire().await.map_err(eyre::Report::new)?;

        Ok(conn)
    }

    pub async fn from_database_url(database_url: &str) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .connect(database_url)
            .await
            .map_err(eyre::Report::new)?;

        Ok(Self { pool })
    }

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl TryFrom<GameRow> for Game {
    type Error = Error;

    fn try_from(row: GameRow) -> Result<Self, Self::Error> {
        let GameRow {
            id,
            start_time,
            end_time,
            points_limit,
            players_number,
            rounds,
        } = row;

        let players_number = players_number.try_into().map_err(eyre::Report::new)?;
        let points_limit = points_limit.try_into().map_err(eyre::Report::new)?;

        Game::load(LoadGameParameters {
            id,
            rounds: rounds
                .iter()
                .map(TryInto::<Round>::try_into)
                .collect::<Result<Vec<Round>, Error>>()?,
            end_time,
            start_time,
            players_number: Number::new(players_number)?,
            points_limit: Points::new(points_limit),
        })
    }
}

impl TryFrom<&RoundsColumn> for Round {
    type Error = Error;

    fn try_from(value: &RoundsColumn) -> Result<Self, Self::Error> {
        let RoundsColumn {
            round_number,
            player_number,
            points_kind,
            points,
        } = value.clone();

        let round_number = round_number.try_into().map_err(eyre::Report::new)?;
        let player_number = player_number.try_into().map_err(eyre::Report::new)?;
        let player_score = player_score(points, points_kind)?;

        Ok(Self::new(NewRoundParameters {
            number: Number::new(round_number)?,
            player_number: Number::new(player_number)?,
            player_score,
        }))
    }
}

fn player_score(points: i32, points_kind: String) -> Result<PlayerScore, Error> {
    let points = points.try_into().map_err(eyre::Report::new)?;

    if points_kind == POINTS_KIND_REGULAR {
        return Ok(PlayerScore::Regular(Score::new(points)?));
    }

    if points_kind == POINTS_KIND_OVERTHROW {
        return Ok(PlayerScore::Overthrow(Score::new(points)?));
    }

    Err(Error::Unexpected(eyre::eyre!("Invalid points kind")))
}

impl From<&Game> for NewGameRow {
    fn from(value: &Game) -> Self {
        let rounds: Vec<RoundsColumn> = value.rounds().iter().map(Into::into).collect();

        Self {
            start_time: value.start_time(),
            end_time: value.end_time(),
            points_limit: value.points_limit().value().into(),
            players_number: value.players_number().value() as i32,
            rounds: Json::from(rounds),
        }
    }
}

impl From<&Game> for GameRow {
    fn from(value: &Game) -> Self {
        let rounds: Vec<RoundsColumn> = value.rounds().iter().map(Into::into).collect();

        Self {
            start_time: value.start_time(),
            end_time: value.end_time(),
            points_limit: value.points_limit().value().into(),
            players_number: value.players_number().value() as i32,
            rounds: Json::from(rounds),
            id: value.id().unwrap(),
        }
    }
}

impl From<&Round> for RoundsColumn {
    fn from(value: &Round) -> Self {
        Self {
            round_number: value.number().value() as i32,
            player_number: value.player_number().value() as i32,
            points_kind: match value.player_score() {
                PlayerScore::Regular(_) => POINTS_KIND_REGULAR.to_string(),
                PlayerScore::Overthrow(_) => POINTS_KIND_OVERTHROW.to_string(),
            },
            points: value.player_score().points().value() as i32,
        }
    }
}
