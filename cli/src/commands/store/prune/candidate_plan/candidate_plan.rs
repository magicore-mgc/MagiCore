//! Builds a fail-closed package deletion plan inside a validated virtual store.
//! Tạo kế hoạch xóa fail-closed trong virtual store đã xác thực.

use std::path::{Path, PathBuf};

use mgc_types::PackageId;

use super::error::CandidatePlanError;
use crate::commands::safe_filesystem::{existing_directory_under, logical_payload_bytes};

pub(in crate::commands::store::prune) struct PruneCandidate {
    pub(in crate::commands::store::prune) id: PackageId,
    pub(in crate::commands::store::prune) directory: Option<PathBuf>,
    pub(in crate::commands::store::prune) logical_bytes: u64,
}

pub(in crate::commands::store::prune) fn plan_candidates(
    packages: Vec<PackageId>,
    vstore_root: Option<&Path>,
) -> Result<(Vec<PruneCandidate>, u64), CandidatePlanError> {
    let mut plan = Vec::with_capacity(packages.len());
    let mut projected_total = 0u64;
    for id in packages {
        let directory = match vstore_root {
            Some(root) => candidate_directory(root, &id)?,
            None => None,
        };
        let logical_bytes = match directory.as_deref() {
            Some(path) => logical_payload_bytes(path)?,
            None => 0,
        };
        projected_total = projected_total.checked_add(logical_bytes).ok_or_else(|| {
            CandidatePlanError::PayloadTotalOverflow {
                package: id.to_string(),
            }
        })?;
        plan.push(PruneCandidate {
            id,
            directory,
            logical_bytes,
        });
    }
    Ok((plan, projected_total))
}

fn candidate_directory(
    vstore_root: &Path,
    id: &PackageId,
) -> Result<Option<PathBuf>, CandidatePlanError> {
    let component = format!("{}@{}", id.name_str().replace('/', "+"), id.version());
    if component == "."
        || component == ".."
        || component
            .chars()
            .any(|character| matches!(character, '/' | '\\' | ':' | '\0'))
    {
        return Err(CandidatePlanError::InvalidCandidateComponent {
            package: id.to_string(),
            path: component,
        });
    }
    let candidate = vstore_root.join(&component);
    if candidate.parent() != Some(vstore_root) {
        return Err(CandidatePlanError::InvalidCandidateComponent {
            package: id.to_string(),
            path: display_path(&candidate),
        });
    }
    Ok(existing_directory_under(&candidate, vstore_root)?)
}

fn display_path(path: &Path) -> String {
    path.display().to_string()
}
