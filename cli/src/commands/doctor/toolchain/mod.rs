//! Tool discovery and version reporting.
//! Tìm tool và báo cáo version.

mod toolchain;

pub(super) use toolchain::{TOOLCHAIN_PROBES, inventory, inventory_for_project};

#[cfg(test)]
pub(super) use toolchain::inventory_for_core;

#[cfg(test)]
pub(super) use toolchain::{
    MAX_VERSION_LINE_BYTES, VERSION_PROBE_ENV_KEYS, VERSION_TRUNCATION_MARKER, VERSION_UNAVAILABLE,
    pin_candidate, safe_path_display, version_from_stdout, version_probe_environment,
    version_probe_options, which_in_path, which_in_paths,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
