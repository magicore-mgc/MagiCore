//! OWASP fixture schema wiring.
//! File này chỉ export type schema cho parser regression.

mod schema;

pub(crate) use schema::{OwaspReport, OwaspVulnerability};
