use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::*;
use types::*;
use warp::Filter;

mod filters;
mod handlers;
mod models;
mod poker;
mod types;

const PORT: u16 = 8080;

pub type Db = Pool<Postgres>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            std::env::var("DATABASE_URL")
                .expect("missing required environment variable 'DATABASE_URL'")
                .as_str(),
        )
        .await?;

    info!("connected to database");

    let state = State::default();
    let state = Arc::new(Mutex::new(state));
    let routes = filters::index(db).with(warp::trace::request());

    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let (addr, server) = warp::serve(routes)
        .bind_with_graceful_shutdown(addr, async { tokio::signal::ctrl_c().await.unwrap() });

    info!("started listening on {:?}", addr);

    tokio::spawn(server).await.expect("couldn't start server");
    Ok(())
}
