//! Wire the bounded retry loop and its deadline poller.
//! Nối vòng retry có giới hạn và poller deadline.

#[path = "retry.rs"]
mod retry;
#[path = "wait.rs"]
mod wait;
pub(in crate::run::implementation) use retry::retry_windows_process_scan;
pub(super) use wait::wait_for_windows_process_retry;
