//! lease column probe module wiring.
//! Wiring module lease column probe.
#[path = "lease_columns_present.rs"]
mod lease_columns_present;
pub(in crate::database::database::schema) use lease_columns_present::lease_columns_present;
