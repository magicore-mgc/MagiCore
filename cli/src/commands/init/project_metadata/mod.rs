//! Project metadata owner exports.
//! Export owner metadata dự án.

mod project_metadata;

pub(in crate::commands::init) use project_metadata::write_mgc_toml;
