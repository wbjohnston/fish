use crossbeam_channel::bounded as channel;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::net::SocketAddr;
use tracing::*;
use warp::Filter;

mod config;
mod context;
mod filters;
mod handlers;
mod models;
mod prelude;
mod services;
const PORT: u16 = 8080;

use prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env()?;

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(config.database_url.as_str())
        .await?;

    let mut listener = sqlx::postgres::PgListener::connect_with(&db).await?;

    let (tx, rx) = channel(256);

    let context = context::Context {
        db,
        table_notifcations_rx: rx,
    };

    tokio::spawn(async move {
        listener.listen("table_notifications").await.unwrap();

        while let Ok(msg) = listener.recv().await {
            tx.send(msg).unwrap();
        }
    });

    info!("connected to database");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Content-Type",
        ])
        .allow_methods(vec!["POST", "GET"]);

    let routes = filters::index(context)
        .with(warp::trace::request())
        .with(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let (addr, server) = warp::serve(routes)
        .bind_with_graceful_shutdown(addr, async { tokio::signal::ctrl_c().await.unwrap() });

    info!("started listening on {:?}", addr);

    tokio::spawn(server).await.expect("couldn't start server");
    Ok(())
}
