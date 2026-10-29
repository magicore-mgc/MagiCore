//! Canonical core catalog — catalog core chuẩn.
//! Maps the public CLI core names to feature-enabled adapter declarations.
//! Ánh xạ tên core CLI sang khai báo adapter được bật theo feature.

use anyhow::Result;
use mgc_types::capabilities::Capability;

use crate::commands::capabilities::error::CapabilitiesError;

/// CLI-canonical core names (matrix-lane naming: cloud = "clo").
/// Tên core chuẩn của CLI (đặt tên theo lane matrix: cloud = "clo").
pub(in crate::commands::capabilities) const ALL_CORES: [&str; 9] = [
    "web", "lib", "ai", "app", "game", "iot", "clo", "cicd", "hardware",
];

/// Static capability manifest per core — no adapter construction needed
/// (detection is not required to READ the claims).
/// Bảng capability tĩnh theo core — không cần dựng adapter (đọc claim không
/// cần detect project).
pub(in crate::commands::capabilities) fn capabilities_for_core(
    core: &str,
) -> Result<&'static [Capability]> {
    let caps: &'static [Capability] = match core {
        #[cfg(feature = "web")]
        "web" => mgc_web_adapter::WebAdapter::CAPABILITIES,
        // Lib narrows its claims per language (`CAPABILITIES` for native,
        // `CAPABILITIES_TS`/`_JAVA_DELEGATED`/`_PYTHON_UNOWNED` for the rest).
        // Printing only the base set reported LESS than a real project performs:
        // `mgc outdated` on a Python project works through the resolver, yet the
        // matrix said `lib` had no `dependency-resolver`. The union is the
        // truthful "what this core can do for SOME project".
        // (Lib thu hẹp claim theo ngôn ngữ; in mỗi bản gốc báo THẤP hơn thực tế.)
        #[cfg(feature = "lib")]
        "lib" => mgc_lib_adapter::LibAdapter::CAPABILITIES_ALL,
        // AI is the same shape: the base set plus the PYPI lane, which is what an
        // AI project (`pyproject.toml`) actually gets.
        // (AI cũng vậy: bản gốc cộng lane PYPI mà project AI thật sự nhận.)
        #[cfg(feature = "ai")]
        "ai" => mgc_ai_adapter::AiAdapter::CAPABILITIES_ALL,
        #[cfg(feature = "app")]
        "app" => mgc_app_adapter::AppAdapter::CAPABILITIES,
        #[cfg(feature = "game")]
        "game" => mgc_game_adapter::GameAdapter::CAPABILITIES,
        #[cfg(feature = "iot")]
        "iot" => mgc_iot_adapter::IotAdapter::CAPABILITIES,
        #[cfg(feature = "clo")]
        "clo" => mgc_cloud_adapter::CloudAdapter::CAPABILITIES,
        #[cfg(feature = "cicd")]
        "cicd" => mgc_cicd_adapter::CicdAdapter::CAPABILITIES,
        #[cfg(feature = "hardware")]
        "hardware" => mgc_hardware_adapter::HardwareAdapter::CAPABILITIES,
        other => {
            return Err(CapabilitiesError::UnknownCore {
                core: other.to_owned(),
            }
            .into());
        }
    };
    Ok(caps)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
