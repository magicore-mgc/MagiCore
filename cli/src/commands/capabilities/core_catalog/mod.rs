//! Core catalog wiring — nối owner catalog core.
//! The child owns the canonical names and adapter mapping.
//! File con sở hữu tên chuẩn và ánh xạ adapter.

mod core_catalog;

pub(super) use core_catalog::{ALL_CORES, capabilities_for_core};
