use std::net::SocketAddr;
use std::sync::Arc;
use std::{collections::HashMap, convert::Infallible};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use tracing::*;
use types::*;

mod filters;
mod handlers;
mod poker;
mod types;

const PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = State::default();
    let state = Arc::new(Mutex::new(state));
    let routes = filters::index(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let (addr, server) = warp::serve(routes)
        .bind_with_graceful_shutdown(addr, async { tokio::signal::ctrl_c().await.unwrap() });

    info!("started listening on {:?}", addr);

    tokio::spawn(server).await;
}
