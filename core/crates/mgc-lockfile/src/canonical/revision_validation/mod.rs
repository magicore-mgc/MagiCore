//! v4 revision validation owner wiring.
//! File này chỉ export kiểm tra revision lockfile v4.

mod revision_validation;

pub(crate) use revision_validation::validate_v4_revision;
