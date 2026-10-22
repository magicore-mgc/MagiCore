//! Map the host and build variant to release artifact coordinates.
//! Ánh xạ hệ điều hành, kiến trúc và biến thể build thành tọa độ artifact.

use crate::commands::self_update::error::SelfUpdateError;
use anyhow::Result;

/// This host in contract labels (`linux|macos|windows`, `x64|arm64`).
/// Unknown platforms fail closed with the supported list.
/// (OS/arch máy này theo nhãn contract.)
pub fn host_labels() -> Result<(String, String, String)> {
    let os = match std::env::consts::OS {
        "linux" => "linux",
        "macos" => "macos",
        "windows" => "windows",
        other => {
            return Err(SelfUpdateError::unsupported_host(&format!("OS {other}")));
        }
    };
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x64",
        "aarch64" => "arm64",
        other => {
            return Err(SelfUpdateError::unsupported_host(&format!("arch {other}")));
        }
    };
    let ext = if os == "windows" { "zip" } else { "tar.gz" };
    Ok((os.to_string(), arch.to_string(), ext.to_string()))
}

/// Artifact file name — the SAME rule as
/// scripts/release-artifact-contract.sh (`{pkg}-{version}-{os}-{arch}.{ext}`,
/// all lowercase, version without `v`). Duplicated here (like install.ps1
/// / install-from-gh.sh) because an installed binary has no repo checkout
/// to call the script from; unit tests pin sample outputs to the contract.
/// (Tên artifact — cùng quy tắc với contract script.)
pub fn artifact_name(package: &str, version: &str, os: &str, arch: &str, ext: &str) -> String {
    format!("{package}-{version}-{os}-{arch}.{ext}")
}

/// Which release variant this binary is: `all` when every core feature
/// is compiled in, else `web`. Determined at compile time, never guessed
/// at runtime (a wrong variant 404s).
/// (Variant của binary này — xác định lúc compile.)
pub(in crate::commands) fn detect_variant() -> String {
    #[cfg(all(
        feature = "web",
        feature = "game",
        feature = "ai",
        feature = "clo",
        feature = "cicd",
        feature = "iot",
        feature = "app",
        feature = "lib",
        feature = "hardware"
    ))]
    {
        "magicore".to_string()
    }
    #[cfg(not(all(
        feature = "web",
        feature = "game",
        feature = "ai",
        feature = "clo",
        feature = "cicd",
        feature = "iot",
        feature = "app",
        feature = "lib",
        feature = "hardware"
    )))]
    {
        "magicore-web".to_string()
    }
}
