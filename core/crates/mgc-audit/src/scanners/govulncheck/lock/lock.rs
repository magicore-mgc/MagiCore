//! Go pins from the MGC lockfile, so audit covers the whole resolved graph.
//! Pin Go từ lockfile MGC để audit phủ toàn bộ graph đã resolve.
//!
//! The OSV fallback used to read `go.mod` only, which holds DIRECT requires; an
//! advisory against a transitive module was therefore invisible and every Go
//! audit reported `Partial`. MagiCore already resolved and locked the full graph
//! (`mgc install` writes it), so reading the lock is both cheaper and complete.
//! (`go.mod` chỉ có require TRỰC TIẾP nên advisory cho module bắc cầu vô hình và
//! mọi audit Go báo `Partial`. MagiCore đã resolve và lock toàn bộ graph, nên đọc
//! lock vừa rẻ vừa đủ.)

use crate::scanners::osv::OsvPin;
use mgc_types::{MgError, MgResult};
use std::path::Path;

/// Ecosystem string OSV uses for Go modules.
/// Chuỗi ecosystem OSV dùng cho Go module.
pub(in crate::scanners::govulncheck) const GO_OSV_ECOSYSTEM: &str = "Go";

/// Read every Go module pinned in `mgc.lock`.
///
/// `Ok(None)` means there is no lockfile at all, which lets the caller fall back
/// to `go.mod`. A lockfile that exists but cannot be read or parsed is an error:
/// silently auditing `go.mod` instead would report less coverage than the project
/// actually has, and the report would look like a clean result.
/// (`Ok(None)` = không có lock → caller fallback về `go.mod`. Lock có nhưng không
/// đọc/parse được là lỗi: âm thầm audit `go.mod` sẽ báo coverage thấp hơn thực tế
/// và trông như kết quả sạch.)
pub(in crate::scanners::govulncheck) fn go_pins_from_mgc_lock(
    project_root: &Path,
) -> MgResult<Option<Vec<OsvPin>>> {
    let path = project_root.join("mgc.lock");
    let bytes = match mgc_lockfile::parser::read_lockfile_bytes(&path) {
        Ok(bytes) => bytes,
        Err(mgc_lockfile::LockfileError::IoError(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            return Ok(None);
        }
        Err(error) => {
            return Err(MgError::Other(format!(
                "cannot read MGC lockfile for Go audit at {}: {error}",
                path.display()
            )));
        }
    };
    let text = String::from_utf8(bytes).map_err(|error| {
        MgError::Other(format!(
            "MGC lockfile is not valid UTF-8 at {}: {error}",
            path.display()
        ))
    })?;

    if mgc_lockfile::detect_lockfile_version(&text).is_ok_and(|version| version == 4) {
        let doc = mgc_lockfile::canonical::parse_v4_document(&text).map_err(|error| {
            MgError::Other(format!(
                "cannot parse v4 MGC lockfile for Go audit at {}: {error}",
                path.display()
            ))
        })?;
        return Ok(Some(
            doc.packages
                .iter()
                .filter(|package| package.key.ecosystem == mgc_lockfile::EcosystemTag::Go)
                .map(|package| OsvPin {
                    name: package.key.name.clone(),
                    version: package.key.version.clone(),
                    ecosystem: GO_OSV_ECOSYSTEM,
                })
                .collect(),
        ));
    }

    let lockfile = match mgc_lockfile::parser::load_lockfile(&path) {
        Ok(lockfile) => lockfile,
        Err(mgc_lockfile::LockfileError::IoError(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            return Ok(None);
        }
        Err(error) => {
            return Err(MgError::Other(format!(
                "cannot read MGC lockfile for Go audit at {}: {error}",
                path.display()
            )));
        }
    };
    Ok(Some(
        lockfile
            .packages
            .iter()
            .filter(|package| package.ecosystem == mgc_lockfile::EcosystemTag::Go)
            .map(|package| OsvPin {
                name: package.name.clone(),
                version: package.version.clone(),
                ecosystem: GO_OSV_ECOSYSTEM,
            })
            .collect(),
    ))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
