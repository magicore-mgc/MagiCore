use super::super::*;
use super::HardwareBuildError;

/// Hardware PlatformIO projects require a MagiCore-owned build backend.
/// Project PlatformIO của Hardware cần backend build do MagiCore sở hữu.
#[cfg(feature = "hardware")]
pub(in crate::commands::build::implementation) async fn build_hardware(root: &Path) -> Result<()> {
    if project_manifest_present(root, "platformio.ini")? {
        return Err(HardwareBuildError::Unsupported {
            target: "hardware/platformio",
            guidance: "MagiCore has no native PlatformIO build backend; refusing to invoke `pio run`, which may resolve or install dependencies",
        }
        .into());
    }
    Err(HardwareBuildError::FrameworkNotDetected {
        root: root.to_path_buf(),
    }
    .into())
}
