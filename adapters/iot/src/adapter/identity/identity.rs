//! Implements the stable IoT core identity and ecosystem label. — Implement định danh core IoT và nhãn ecosystem ổn định.

use super::super::IotAdapter;
use mgc_types::Ecosystem;
use mgc_types::capabilities::CoreIdent;
impl CoreIdent for IotAdapter {
    fn core_id(&self) -> &'static str {
        "iot"
    }

    fn name(&self) -> &str {
        "iot"
    }

    fn ecosystem(&self) -> Ecosystem {
        Ecosystem::Iot
    }
}
