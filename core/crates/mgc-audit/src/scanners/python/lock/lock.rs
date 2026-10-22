use mgc_types::{MgError, MgResult};
use std::path::Path;

use crate::scanners::osv;

/// Read MGC-owned Python pins. `None` means no mgc.lock exists; a present but
/// malformed lock is an error and an empty Python graph remains authoritative.
/// Đọc pin Python do MGC sở hữu. `None` là chưa có mgc.lock; lock hỏng là lỗi,
/// còn graph Python rỗng trong lock hợp lệ vẫn là nguồn dữ liệu có thẩm quyền.
pub fn python_pins_from_mgc_lock(project_root: &Path) -> MgResult<Option<Vec<osv::OsvPin>>> {
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
                "cannot read MGC lockfile for Python audit at {}: {error}",
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
    // v4 documents expose Python instances directly (canonical versions
    // audit-match at least as well as raw specifiers); other versions
    // keep the legacy reader below, which owns their errors.
    // (Tài liệu v4 cho pin Python trực tiếp; version khác giữ reader cũ.)
    if mgc_lockfile::detect_lockfile_version(&text).is_ok_and(|version| version == 4) {
        let doc = mgc_lockfile::canonical::parse_v4_document(&text).map_err(|error| {
            MgError::Other(format!(
                "cannot parse v4 MGC lockfile for Python audit at {}: {error}",
                path.display()
            ))
        })?;
        return Ok(Some(
            doc.packages
                .iter()
                .filter(|p| p.key.ecosystem == mgc_lockfile::EcosystemTag::Python)
                .map(|p| osv::OsvPin {
                    name: p.key.name.clone(),
                    version: p.key.version.clone(),
                    ecosystem: "PyPI",
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
                "cannot read MGC lockfile for Python audit at {}: {error}",
                path.display()
            )));
        }
    };
    Ok(Some(
        lockfile
            .packages
            .iter()
            .filter(|p| p.ecosystem == mgc_lockfile::EcosystemTag::Python)
            .map(|p| osv::OsvPin {
                name: p.name.clone(),
                version: p.version.clone(),
                ecosystem: "PyPI",
            })
            .collect(),
    ))
}
