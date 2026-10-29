//! `lockfile` — Web adapter lockfile API and owner wiring.
//! API lock Web ổn định và wiring các owner theo chức năng.
mod atomic_write;
mod cache_dir;
mod checksum;
mod edge_target;
mod error;
mod graph_match;
mod graph_restore;
mod installed_package;
mod lockfile;
mod manifest_match;
mod owner_core;
mod reader;
mod v3_writer;
mod v4_reader;
mod v4_writer;

pub use cache_dir::project_cache_dir;
pub use checksum::{
    allow_unsigned_integrity, compute_sha512_b64, compute_tarball_integrity,
    compute_tarball_integrity_reader, maybe_warn_missing_lockfile_checksum,
    strict_integrity_enforced,
};
pub use graph_match::web_lockfile_matches_graph;
pub use graph_restore::build_graph_from_lockfile;
pub use installed_package::{installed_package_matches, installed_package_version};
pub use lockfile::write_web_lockfile_with_state;
pub use manifest_match::lockfile_satisfies_manifest;
pub use reader::{read_web_lockfile, read_web_lockfile_checked};

pub(crate) use atomic_write::atomic_write_web_lockfile;
pub(crate) use edge_target::locked_edge_target;
pub(crate) use error::LockfileError;
pub(crate) use graph_match::{is_web_lock_package, resolved_graphs_match_material};
pub(crate) use owner_core::web_lock_owner_core;
pub(crate) use reader::{lock_on_disk_is_v4, read_web_lockfile_document, scope_web_lock_packages};
pub(crate) use v3_writer::{fresh_v3_lockfile, web_lock_schema_is_v4};
pub(crate) use v4_reader::{read_v4_document_if_present, verify_v4_document_integrity};
pub(crate) use v4_writer::write_web_lockfile_v4;
