// Advisory response owners. / Các owner response advisory.
mod findings;
mod parse;
mod semver_range;
mod type_of_json;
mod types;

pub(super) use findings::build_findings_for_advisory;
pub use parse::parse_advisory_bulk_response;
pub(super) use semver_range::validate_semver_range;
pub(super) use type_of_json::type_of_json;
pub(super) use types::AdvisoryRecord;
#[cfg(test)]
pub(crate) use types::is_cve_identifier;

#[cfg(test)]
#[path = "test/response.rs"]
mod tests;
