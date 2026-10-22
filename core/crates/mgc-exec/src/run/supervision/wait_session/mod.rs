//! Child wait-session owners and their compatibility entry point.
//! Owner của phiên chờ process con và entry point tương thích.

#[path = "cancellation/mod.rs"]
mod cancellation;
#[path = "wait_session.rs"]
mod implementation;
#[path = "output_capture/mod.rs"]
mod output_capture;
#[path = "process_scan/mod.rs"]
mod process_scan;
#[path = "root_exit/mod.rs"]
mod root_exit;
#[path = "termination/mod.rs"]
mod termination;
#[path = "timeout/mod.rs"]
mod timeout;
#[path = "wait_loop/mod.rs"]
mod wait_loop;

pub(in crate::run::implementation) use wait_loop::wait_with_timeout;
