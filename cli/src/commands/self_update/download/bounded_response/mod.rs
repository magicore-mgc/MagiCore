//! Export the bounded response reader.
//! Công bố reader response có quota.

mod bounded_response;
pub(in crate::commands) use bounded_response::download_capped;
