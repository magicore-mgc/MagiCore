//! Wire the npm advisory request, typed errors and query outcomes.
//! Nối request advisory npm, lỗi typed và query outcome.

#[path = "advisory_query.rs"]
mod advisory_query;
#[path = "error.rs"]
mod error;
#[path = "outcome.rs"]
mod outcome;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
pub(super) use advisory_query::query_npm_advisories;
pub(super) use error::AdvisoryQueryError;
pub(super) use outcome::AdvisoryQueryOutcome;
