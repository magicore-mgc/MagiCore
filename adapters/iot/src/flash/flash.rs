//! Keeps firmware flashing explicitly unsupported without an MGC-owned backend.
//! Giữ flash firmware ở trạng thái unsupported khi chưa có backend của MGC.

use std::path::Path;

use crate::framework::IotFramework;
use mgc_types::MgResult;

use super::flash_result::FlashResult;

/// Flash firmware only when a MagiCore-owned backend exists.
/// Chỉ flash firmware khi có backend do MagiCore sở hữu.
pub async fn flash_firmware(
    framework: IotFramework,
    _project_root: &Path,
    _port: Option<&str>,
) -> MgResult<FlashResult> {
    Err(mgc_types::capabilities::unsupported_capability(
        "iot",
        "flash",
        &format!(
            "{} has no MagiCore-owned firmware flashing backend; external flash tools are not invoked",
            framework.as_str()
        ),
    ))
}
