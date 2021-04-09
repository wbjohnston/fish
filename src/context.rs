use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Context {
    pub db: Db,
    pub table_notifcations_rx: crossbeam_channel::Receiver<sqlx::postgres::PgNotification>,
}
