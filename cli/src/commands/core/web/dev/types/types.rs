//! Internal launch, package-manifest and target types shared by Web dev owners.
//! Kiểu launch, manifest package và target nội bộ dùng chung trong các owner Web dev.

use serde::Deserialize;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug)]
pub(in crate::commands::core::web::implementation) struct DevLaunch {
    pub(in crate::commands::core::web::implementation) program: PathBuf,
    pub(in crate::commands::core::web::implementation) args: Vec<OsString>,
    pub(in crate::commands::core::web::implementation) envs: Vec<(OsString, OsString)>,
}

impl DevLaunch {
    pub(in crate::commands::core::web::implementation) fn describe(&self) -> String {
        let args = self
            .args
            .iter()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");
        if args.is_empty() {
            self.program.display().to_string()
        } else {
            format!("{} {}", self.program.display(), args)
        }
    }
}

#[derive(Debug, Deserialize)]
pub(in crate::commands::core::web::implementation) struct DevPackageJson {
    pub(in crate::commands::core::web::implementation) scripts:
        Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub(in crate::commands::core::web::implementation) struct DevTarget {
    pub(in crate::commands::core::web::implementation) dir: PathBuf,
    pub(in crate::commands::core::web::implementation) role: &'static str,
    pub(in crate::commands::core::web::implementation) script_name: &'static str,
    pub(in crate::commands::core::web::implementation) host: Option<String>,
    pub(in crate::commands::core::web::implementation) port: Option<u16>,
}
