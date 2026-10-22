//! Validate lock coverage and existing lock safety before native Lib install.
//! Xác minh coverage và độ an toàn lock trước install native Lib.

use crate::install::{cargo_compat, ecosystem_for_language, validate_existing_lock_for_install};
use crate::language::LibLanguage;
use mgc_lockfile::Package;
use mgc_types::{MgResult, ResolvedGraph};
use std::path::Path;

pub(in crate::install) fn validate_native_install_inputs(
    language: LibLanguage,
    graph: &ResolvedGraph,
    project_root: &Path,
    lock_packages: &[Package],
) -> MgResult<()> {
    crate::install::validate_lock_coverage(graph, ecosystem_for_language(language), lock_packages)?;
    if language == LibLanguage::Rust {
        cargo_compat::validate_install(project_root, lock_packages, graph)?;
    }
    // Reject a malformed or hostile lock before downloads/CAS writes.
    // V4 has its own digest-aware merge reader; never feed it through
    // the legacy V3 model. (Từ chối lock hỏng trước khi tải/ghi CAS;
    // V4 xác minh qua reader digest riêng, không parse bằng model V3.)
    validate_existing_lock_for_install(project_root)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
