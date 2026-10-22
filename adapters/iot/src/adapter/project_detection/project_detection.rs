//! Maps the existing IoT framework manifest detector to ProjectDetector. Detection rules remain in the framework owner. — Nối detector manifest IoT hiện có vào ProjectDetector. Quy tắc detect vẫn do framework owner sở hữu.

use std::path::Path;

use super::super::IotAdapter;
use crate::framework::manifest_is_iot;
use mgc_types::capabilities::ProjectDetector;
impl ProjectDetector for IotAdapter {
    fn can_handle(&self, project_root: &Path) -> bool {
        manifest_is_iot(project_root)
    }
}
