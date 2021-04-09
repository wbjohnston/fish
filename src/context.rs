use crate::{models::notification::Notification, prelude::*};
use tokio::sync::broadcast::Sender;

#[derive(Debug, Clone)]
pub struct Context {
    pub db: Db,
    pub tx: Sender<Notification>,
}
