//! Swift Package.resolved parser cases.
//! Các case parser Package.resolved của Swift.

use mgc_audit::scanners::read_swift_resolved;

#[test]
fn swift_resolved_v1_and_v2_shapes_parse() {
    // v2 (Xcode 14+): pins at the root; OSV `SwiftURL` names are GIT
    // URLS (verified live: {"ecosystem":"swift"} → 400).
    // v2: ghim ở root; name `SwiftURL` của OSV là URL GIT (đã verify
    // sống: "swift" → 400).
    let v2 = r#"{
  "pins": [
    {"identity": "swift-nio-http2", "location": "https://github.com/apple/swift-nio-http2.git", "state": {"version": "1.40.0"}},
    {"identity": "swift-custom", "location": "https://github.com/x/swift-custom.git", "state": {"branch": "main", "revision": "abc123"}}
  ],
  "version": 2
}"#;
    let (pins, skipped) = read_swift_resolved(v2).unwrap();
    assert_eq!(pins.len(), 1);
    // NAMING FIX (2026-09-10): OSV `SwiftURL` names are the repo path
    // WITHOUT scheme and .git — `github.com/owner/repo`. The full git
    // URL form queries EMPTY on the live API (fake clean); the
    // normalized form hits GHSA-q3g2-m552-3r9c + GHSA-4px2-pw77-vc85
    // at version 1.40.0 (verified live).
    // SỬA TÊN: tên OSV `SwiftURL` là đường dẫn repo KHÔNG scheme
    // KHÔNG .git. URL git đầy đủ query RỖNG trên API sống (sạch giả);
    // dạng chuẩn hóa dính 2 GHSA thật tại 1.40.0 (verify sống).
    assert_eq!(
        pins[0].name, "github.com/apple/swift-nio-http2",
        "OSV SwiftURL name must be the normalized repo path, not the raw URL"
    );
    assert_eq!(pins[0].version, "1.40.0");
    assert_eq!(pins[0].ecosystem, "SwiftURL");
    assert_eq!(skipped.len(), 1, "versionless checkout recorded as skipped");

    // v1: pins nested under object with repositoryURL.
    // v1: ghim nằm trong object kèm repositoryURL.
    let v1 = r#"{
  "object": {"pins": [{"package": "alamofire", "repositoryURL": "https://github.com/Alamofire/Alamofire.git", "state": {"version": "5.9.1"}}]}
}"#;
    let (pins, _) = read_swift_resolved(v1).unwrap();
    assert_eq!(pins.len(), 1);
    assert_eq!(
        pins[0].name, "github.com/Alamofire/Alamofire",
        "v1 repositoryURL normalizes the same way"
    );
    assert_eq!(pins[0].ecosystem, "SwiftURL");

    // A v2 pin WITHOUT a location cannot be queried — skipped honestly.
    // Ghim v2 THIẾU location thì không truy vấn được — bỏ trung thực.
    let no_loc = r#"{
  "pins": [
    {"identity": "orphan-pin", "state": {"version": "1.0.0"}}
  ],
  "version": 2
}"#;
    let (pins, skipped) = read_swift_resolved(no_loc).unwrap();
    assert!(pins.is_empty());
    assert!(skipped.iter().any(|s| s.contains("orphan-pin")));
}

#[test]
fn swift_resolved_malformed_fails_closed() {
    assert!(read_swift_resolved("not json").is_err());
    assert!(read_swift_resolved("").is_err());
}
