//! Verdict-policy owner wiring — nối owner phân xử verdict.

mod status_resolution;

pub(in crate::aggregate) use status_resolution::resolve_status;
