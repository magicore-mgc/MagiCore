//! Allows the embedded Web lifecycle lane while Terraform remains unsupported by an MGC-owned runner. — Cho phép lane lifecycle Web nhúng; Terraform vẫn chưa có runner do MGC sở hữu.

use mgc_types::MgResult;
use mgc_types::capabilities::LifecycleRunner;

use super::super::CloudAdapter;

impl LifecycleRunner for CloudAdapter {
    fn probe_lifecycle_runner(&self) -> MgResult<()> {
        if self.web.is_some() {
            Ok(())
        } else {
            Err(mgc_types::capabilities::unsupported_capability(
                "cloud",
                "lifecycle_runner",
                "Terraform init/get is not a MagiCore-owned lifecycle runner; package lifecycle is unsupported",
            ))
        }
    }
}
