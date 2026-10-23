//! Canonical lockfile export dispatch.
//! Điều phối export lockfile canonical.

use mgc_types::MgResult;

use crate::schema::Lockfile;

use super::super::cargo_lock::export_cargo_lock;
use super::super::go_sum::export_go_sum;
use super::super::package_lock::export_package_lock;
use super::super::requirements_txt::export_requirements_txt;
use super::super::target::ExportTarget;

pub fn export_canonical(lock: &Lockfile, target: ExportTarget) -> MgResult<Vec<u8>> {
    match target {
        ExportTarget::CargoLock => export_cargo_lock(lock),
        ExportTarget::RequirementsTxt => export_requirements_txt(lock),
        ExportTarget::GoSum => export_go_sum(lock),
        ExportTarget::PackageLock => export_package_lock(lock),
    }
}
