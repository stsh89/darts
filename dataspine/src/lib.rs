mod games;
mod postgres;
mod repo;
mod scores;

use sqlx::Error;

pub use repo::Repo;

pub(crate) use games::{FindGame, GameRow, InsertGame};
pub(crate) use scores::{DeleteScore, InsertScore, InsertScoreParameters, ScoreRow};

pub(crate) type EmptyResult = Result<(), Error>;
pub(crate) type MaybeRowResult<T> = Result<Option<T>, Error>;
pub(crate) type RowResult<T> = Result<T, Error>;
pub(crate) type RowsResult<T> = Result<Vec<T>, Error>;
