use super::super::xcode_available;

/// Nền tảng đích khi chạy `mgc dev app`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetPlatform {
    /// iOS Simulator (chỉ macOS + Xcode)
    IosSimulator,
    /// Android Emulator / Device
    Android,
}

/// Chọn nền tảng đích dựa theo OS host và tool có sẵn.
/// macOS + Xcode → iOS Simulator; còn lại → Android.
pub fn detect_target_platform() -> TargetPlatform {
    if cfg!(target_os = "macos") && xcode_available() {
        TargetPlatform::IosSimulator
    } else {
        TargetPlatform::Android
    }
}

#[cfg(test)]
#[path = "test/platform.rs"]
mod tests;
