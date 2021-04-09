use sqlx::{Pool, Postgres};

pub use crate::context::Context;

pub type Db = Pool<Postgres>;
pub type Tx<'a> = sqlx::Transaction<'a, sqlx::Postgres>;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type WebResult<T> = std::result::Result<T, warp::Rejection>;
