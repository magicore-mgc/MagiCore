//! Implement the stable hardware core identity.
//! Triển khai định danh core hardware ổn định.

use super::super::adapter::HardwareAdapter;
use mgc_types::Ecosystem;
use mgc_types::capabilities::CoreIdent;

impl CoreIdent for HardwareAdapter {
    fn core_id(&self) -> &'static str {
        "hardware"
    }

    fn name(&self) -> &str {
        "hardware"
    }

    fn ecosystem(&self) -> Ecosystem {
        Ecosystem::Hardware
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
