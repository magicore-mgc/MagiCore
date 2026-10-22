use rusqlite::Connection;
use std::sync::{Arc, Mutex};
pub struct Database {
    pub(in crate::database::database) conn: Connection,
    pub(in crate::database::database) claim_write_gate: Arc<Mutex<()>>,
}
