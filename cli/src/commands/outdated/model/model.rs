//! Stable serializable outdated package data.
//! Dữ liệu package outdated có contract serialize ổn định.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub(in crate::commands::outdated) struct OutdatedPkg {
    pub(in crate::commands::outdated) name: String,
    pub(in crate::commands::outdated) current: String,
    pub(in crate::commands::outdated) latest: String,
    pub(in crate::commands::outdated) major: u64,
    pub(in crate::commands::outdated) minor: u64,
    pub(in crate::commands::outdated) patch: u64,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
