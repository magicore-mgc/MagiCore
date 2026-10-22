//! Wire Web audit report projections and nearest tests.
//! Nối các projection audit report Web và test gần owner.

#[path = "report.rs"]
mod report;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
pub(super) use report::{report_after_npm_scan, report_without_npm_pins};
