//! requirements.txt skeleton writer.
//! Writer skeleton requirements.txt.

use mgc_types::MgResult;

use crate::ecosystem_tag::EcosystemTag;
use crate::schema::Lockfile;

use super::super::shared::{TEXT_EXPORT_HEADER, require_version};

/// requirements.txt skeleton: one `name==version` pin per python entry.
/// requirements.txt skeleton: một pin `name==version` mỗi entry python.
pub fn export_requirements_txt(lock: &Lockfile) -> MgResult<Vec<u8>> {
    let mut out = String::from(TEXT_EXPORT_HEADER);
    for pkg in lock
        .packages
        .iter()
        .filter(|p| p.ecosystem == EcosystemTag::Python)
    {
        let version = require_version(pkg)?;
        out.push_str(&format!("{}=={version}\n", pkg.name));
    }
    Ok(out.into_bytes())
}
