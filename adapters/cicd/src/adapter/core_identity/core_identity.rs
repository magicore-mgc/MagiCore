//! Implement the stable CI/CD core identity.
//! Triển khai định danh core CICD ổn định.

use super::super::adapter::CicdAdapter;
use mgc_types::Ecosystem;
use mgc_types::capabilities::CoreIdent;

impl CoreIdent for CicdAdapter {
    fn core_id(&self) -> &'static str {
        "cicd"
    }

    fn name(&self) -> &str {
        "cicd"
    }

    fn ecosystem(&self) -> Ecosystem {
        Ecosystem::Cicd
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
