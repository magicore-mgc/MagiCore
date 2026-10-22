use crate::adapter::types::LibAdapter;
use mgc_types::Ecosystem;
use mgc_types::capabilities::CoreIdent;

impl CoreIdent for LibAdapter {
    fn core_id(&self) -> &'static str {
        "lib"
    }

    fn name(&self) -> &str {
        "lib"
    }

    fn ecosystem(&self) -> Ecosystem {
        Ecosystem::Lib
    }
}
