use crate::{
    game_row::{FindGame, InsertGame, ListGames, RoundsColumnItem, UpdateGame},
    GameRow,
};
use playground::{
    coordinator, Error, Game, LoadGameParameters, NewRoundParameters, Number, PlayerScore, Points,
    Round, Score,
};
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, PgPool, Postgres};
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

impl coordinator::InsertGame for Repo {
    async fn insert_game(&self, game: &mut Game) -> Result<(), Error> {
        self.conn().await?.insert_game(game).await
    }
}

impl coordinator::ListGames for Repo {
    async fn list_games(&self) -> Result<Vec<Game>, Error> {
        let games = self
            .conn()
            .await?
            .list_games()
            .await?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<Game>, Error>>()?;

        Ok(games)
    }
}

impl coordinator::UpdateGame for Repo {
    async fn update_game(&self, game: &mut Game) -> Result<(), Error> {
        self.conn().await?.update_game(game).await
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

        Ok(Self::new(pool))
    }

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl TryFrom<GameRow> for Game {
    type Error = Error;

    fn try_from(row: GameRow) -> Result<Self, Self::Error> {
        let GameRow {
            end_time,
            id,
            insert_time,
            players_number,
            points_limit,
            rounds,
            start_time,
            update_time,
        } = row;

        let players_number = players_number.try_into().map_err(eyre::Report::new)?;
        let points_limit = points_limit.try_into().map_err(eyre::Report::new)?;
        let rounds = rounds
            .iter()
            .map(TryInto::<Round>::try_into)
            .collect::<Result<Vec<Round>, Error>>()?;

        Game::load(LoadGameParameters {
            create_time: insert_time,
            end_time,
            id,
            players_number: Number::new(players_number)?,
            points_limit: Points::new(points_limit),
            rounds,
            start_time,
            update_time,
        })
    }
}

impl TryFrom<&RoundsColumnItem> for Round {
    type Error = Error;

    fn try_from(value: &RoundsColumnItem) -> Result<Self, Self::Error> {
        let RoundsColumnItem {
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

impl From<&Round> for RoundsColumnItem {
    fn from(value: &Round) -> Self {
        let (points_kind, points) = match value.player_score() {
            PlayerScore::Regular(score) => (POINTS_KIND_REGULAR, score.points().value().into()),
            PlayerScore::Overthrow(score) => (POINTS_KIND_OVERTHROW, score.points().value().into()),
        };

        Self {
            round_number: value.number().value() as i32,
            player_number: value.player_number().value() as i32,
            points_kind: points_kind.into(),
            points,
        }
    }
}
