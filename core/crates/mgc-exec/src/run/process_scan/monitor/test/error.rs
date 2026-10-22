//! Monitor root-visibility error contract.
//! Contract lỗi root visibility của monitor.

use super::*;

#[test]
fn root_visibility_failure_reports_the_monitored_pid() {
    let error = MonitoredRootNotVisible(42);
    assert_eq!(
        error.to_string(),
        "process table did not contain monitored root PID 42"
    );
}
