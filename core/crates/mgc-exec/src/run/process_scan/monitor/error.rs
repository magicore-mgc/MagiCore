//! Monitor-specific typed process-tree failures.
//! Lỗi cây process typed riêng của monitor.

use std::fmt;

#[derive(Debug)]
pub(in crate::run::implementation) struct MonitoredRootNotVisible(
    pub(in crate::run::implementation) u32,
);

impl fmt::Display for MonitoredRootNotVisible {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "process table did not contain monitored root PID {}",
            self.0
        )
    }
}

impl std::error::Error for MonitoredRootNotVisible {}

#[cfg(test)]
#[path = "test/error.rs"]
mod test;
