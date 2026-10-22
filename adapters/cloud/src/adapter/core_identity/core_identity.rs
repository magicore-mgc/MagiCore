//! Owns the stable `clo` / `cloud` identity and Cloud ecosystem declaration. — Sở hữu định danh ổn định `clo` / `cloud` và khai báo ecosystem Cloud.

use mgc_types::Ecosystem;
use mgc_types::capabilities::CoreIdent;

use super::super::CloudAdapter;

impl CoreIdent for CloudAdapter {
    fn core_id(&self) -> &'static str {
        "clo"
    }

    fn name(&self) -> &str {
        "cloud"
    }

    fn ecosystem(&self) -> Ecosystem {
        Ecosystem::Cloud
    }
}
