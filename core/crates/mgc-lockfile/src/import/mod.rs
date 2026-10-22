//! Legacy lockfile import API — API nhập lockfile cũ.
//! Format parsers are private owners behind this stable facade.

mod bun;
mod deno;
mod detection;
mod import;
mod npm;
mod package_reference;
mod pnpm;
mod types;
mod yarn;

pub use detection::{check_trust_downgrade_risk, detect_legacy_lockfiles};
pub use import::{import_file, import_into_lockfile};
pub use types::{
    ALL, BUN_LOCKFILE, DENO_LOCKFILE, ImportReport, LegacyLockfile, NPM_LOCKFILE, PNPM_LOCKFILE,
    SkippedRecord, YARN_LOCKFILE,
};

pub(in crate::import) use bun::parse_bun;
pub(in crate::import) use deno::parse_deno;
pub(in crate::import) use npm::parse_npm;
pub(in crate::import) use package_reference::{split_name_version, strip_paren_suffix};
pub(in crate::import) use pnpm::parse_pnpm;
pub(in crate::import) use yarn::parse_yarn;
