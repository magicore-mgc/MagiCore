//! Wire OWASP parser regression subjects in original case order.
//! Nối nhóm test hồi quy OWASP theo thứ tự case cũ.

#![allow(clippy::unwrap_used)]

pub(super) use super::parse_owasp_dependency_check_json;

#[path = "concurrent_parse.rs"]
mod concurrent_parse;
#[path = "invalid_shape.rs"]
mod invalid_shape;
#[path = "large_payload.rs"]
mod large_payload;
#[path = "package_identity.rs"]
mod package_identity;
#[path = "report_findings.rs"]
mod report_findings;
