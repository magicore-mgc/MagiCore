//! Writes sorted package, edge, artifact and provenance tables.
//! File này chỉ nối owner package_section.

mod package_section;

pub(crate) use package_section::append_package_section;
