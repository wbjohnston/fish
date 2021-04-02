use std::collections::HashMap;
use std::convert::Infallible;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::*;

use crate::poker::{Command, Table};

pub type ClientId = u32;
pub type RoomId = u32;

#[derive(Debug, Clone, Default)]
pub struct State {
    pub(crate) clients: HashMap<ClientId, Sender<Command>>,
    pub(crate) rooms: HashMap<RoomId, Sender<Command>>,
}

pub struct Room {
    pub(crate) id: RoomId,
    /// Table state
    pub(crate) table: Table,
    pub(crate) client_connections: HashMap<ClientId, Sender<Table>>,
    /// store this so we can clone it for other subscribers
    pub(crate) tx: Sender<Command>,
    /// Inbound commands for room to process
    pub(crate) rx: Receiver<Command>,
}

impl Room {
    pub async fn run(mut self) -> Result<(), Infallible> {
        info!("starting room {}", self.id);
        while let Some(msg) = self.rx.recv().await {
            debug!("received messsage {:?}", msg);
        }

        Ok(())
    }
}
