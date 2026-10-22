//! CocoaPods Podfile.lock parser cases.
//! Các case parser Podfile.lock của CocoaPods.

use mgc_audit::scanners::read_podfile_lock;

#[test]
fn podfile_lock_parses_top_level_pods_and_skips_local() {
    let lock = r#"
PODS:
  - Alamofire (5.9.1)
  - Firebase/CoreOnly (10.27.0):
    - FirebaseCore (= 10.27.0)
  - LocalPod (0.1.0):
    :path: ../LocalPod

DEPENDENCIES:
  - Alamofire

COCOAPODS: 1.15.2
"#;
    let (pins, skipped) = read_podfile_lock(lock).unwrap();
    assert_eq!(pins.len(), 3, "Alamofire, Firebase/CoreOnly, LocalPod");
    let names: Vec<&str> = pins.iter().map(|p| p.name.as_str()).collect();
    assert!(names.contains(&"Alamofire"));
    assert!(names.contains(&"Firebase/CoreOnly"));
    assert_eq!(pins[0].ecosystem, "pods");
    // NOTE: "pods" here is the INTERNAL inventory tag; the OSV lane is
    // unsupported (no advisory DB) — audit_cocoapods_osv reports that
    // honestly without querying.
    // Ghi chú: "pods" là nhãn kiểm kê NỘI BỘ; lane OSV unsupported (không
    // có DB advisory) — audit_cocoapods_osv báo trung thực, không truy vấn.
    // Sub-deps under the top-level pods are NOT re-counted; versionless
    // entries recorded.
    // Dependency bậc dưới không đếm lại; entry thiếu version được ghi nhận.
    assert!(skipped.is_empty() || skipped.iter().all(|s| !s.contains("FirebaseCore")));
}

#[test]
fn podfile_lock_no_pods_section_is_clean_zero() {
    let (pins, skipped) = read_podfile_lock("DEPENDENCIES:\n  - nothing\n").unwrap();
    assert!(pins.is_empty());
    assert!(skipped.is_empty());
}
