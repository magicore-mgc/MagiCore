//! Shared SBOM generation bridge for every adapter.
//! Cầu sinh SBOM dùng chung cho mọi adapter.

use mgc_lockfile::Lockfile;
use mgc_sbom::{SbomGenerator, SbomOptions};
use mgc_types::MgResult;

/// Generate a JSON SBOM from a lockfile.
/// Sinh SBOM JSON từ lockfile.
pub fn generate_sbom(lockfile: &Lockfile, options: SbomOptions) -> MgResult<String> {
    let generator = SbomGenerator::new(options);
    generator
        .generate_json(lockfile)
        .map_err(|e| mgc_types::MgError::Other(format!("SBOM generation failed: {e}")))
}
