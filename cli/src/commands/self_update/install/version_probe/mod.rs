//! Export exact version-token matching.
//! Công bố đối chiếu token version chính xác.

mod version_probe;
pub(in crate::commands) use version_probe::probe_reports_version;
