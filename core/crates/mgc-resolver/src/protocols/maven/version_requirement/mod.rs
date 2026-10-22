//! Maven range matching and version comparison rules.
//! Luật khớp khoảng và so sánh phiên bản Maven.

mod version_requirement;
pub(super) use version_requirement::maven_matches;
