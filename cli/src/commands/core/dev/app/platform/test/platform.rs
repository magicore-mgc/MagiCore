use super::*;

#[test]
fn detect_target_platform_returns_valid_variant() {
    // Không crash — luôn trả về một trong 2 variant hợp lệ
    let platform = detect_target_platform();
    assert!(
        platform == TargetPlatform::IosSimulator || platform == TargetPlatform::Android,
        "expected IosSimulator or Android, got {:?}",
        platform
    );
}

#[cfg(not(target_os = "macos"))]
#[test]
fn non_macos_always_targets_android() {
    // Trên Linux/Windows: platform phải là Android (không bao giờ iOS)
    let platform = detect_target_platform();
    assert_eq!(
        platform,
        TargetPlatform::Android,
        "non-macOS must target Android"
    );
}

#[test]
fn target_platform_debug_format() {
    // Smoke test: Debug trait hoạt động
    let _ = format!("{:?}", TargetPlatform::IosSimulator);
    let _ = format!("{:?}", TargetPlatform::Android);
}
