//! pull_request_target policy route.
//! Route policy pull_request_target.

mod pull_request_target;

pub(super) use pull_request_target::{injection_finding, observe_trigger};
