use models::notification::Notification;
use sqlx::postgres::PgPoolOptions;
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
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env()?;

    let db = PgPoolOptions::new()
        .max_connections(3)
        .connect(config.database_url.as_str())
        .await?;

    let mut listener = sqlx::postgres::PgListener::connect_with(&db).await.unwrap();
    listener.listen("game_notifications").await.unwrap();

    let (tx, _) = tokio::sync::broadcast::channel(128);
    let context = context::Context { db, tx: tx.clone() };

    tokio::spawn(async move {
        while let Ok(msg) = listener.recv().await {
            let notif: Notification = serde_json::from_str(msg.payload()).unwrap();
            tx.send(notif).unwrap();
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
