use super::super::types::Database;
use rusqlite::Connection;

impl Database {
    /// Crate-private raw connection for trusted internal full scans.
    pub(crate) fn conn(&self) -> &Connection {
        &self.conn
    }

    /// Raw connection for integration-test fixture setup only.
    #[cfg(feature = "test-util")]
    #[doc(hidden)]
    pub fn conn_for_tests(&self) -> &Connection {
        &self.conn
    }
}
