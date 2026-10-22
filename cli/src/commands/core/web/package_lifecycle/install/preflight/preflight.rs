//! Validates root identity and offline invocation before install routing.
//! Xác minh root và lời gọi offline trước khi định tuyến cài đặt.

use anyhow::Result;
use std::path::PathBuf;

use super::super::options::WebInstallOptions;
use crate::commands::core::web::implementation::project_context::project_root::project_root;

pub(in crate::commands::core::web::implementation::package_lifecycle::install) fn prepare(
    options: &WebInstallOptions,
) -> Result<PathBuf> {
    let root = project_root()?;
    if options.offline {
        crate::offline::set_offline_mode(true);
        if !options.packages.is_empty() {
            anyhow::bail!("offline install cannot add packages; run `mgc add` online first");
        }
    }
    Ok(root)
}
