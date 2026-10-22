//! lease column addition module wiring.
//! Wiring module lease column addition.
#[path = "add_lease_column.rs"]
mod add_lease_column;
pub(in crate::database::database::schema) use add_lease_column::add_lease_column;
