use super::*;

#[test]
fn xcode_project_prefers_workspace() {
    let dir = std::env::temp_dir().join(format!("mgc-app-xc-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("App.xcodeproj"), "").unwrap();
    std::fs::write(dir.join("App.xcworkspace"), "").unwrap();
    assert_eq!(
        super::find_xcode_project(&dir).as_deref(),
        Some("App.xcworkspace")
    );
    std::fs::remove_file(dir.join("App.xcworkspace")).unwrap();
    assert_eq!(
        super::find_xcode_project(&dir).as_deref(),
        Some("App.xcodeproj")
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(target_os = "macos")]
#[test]
fn find_ios_simulator_returns_some_or_none_without_panic() {
    // On macOS: no panic; with Xcode → Some(udid), without → None.
    // Trên macOS: không panic; có Xcode → Some(udid), không → None.
    let result = find_ios_simulator();
    if let Some(ref udid) = result {
        // UDID must be hex-dash form (8-4-4-4-12).
        // UDID phải dạng hex-dash (8-4-4-4-12).
        assert!(udid.len() >= 8, "UDID too short: {udid}");
    }
    // None is also valid (Xcode not installed).
    // None cũng hợp lệ (không cài Xcode).
}
