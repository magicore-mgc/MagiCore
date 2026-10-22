use super::*;
use mgc_types::Version;

#[test]
fn swift_range_semantics() {
    let v = Version::parse("1.2.3").expect("test operation must succeed");
    assert!(swift_matches("*", &v));
    assert!(swift_matches("1.2.3", &v), "bare = exact pin");
    assert!(!swift_matches("1.2.4", &v));
    assert!(swift_matches("exact:1.2.3", &v));
    assert!(swift_matches("from:1.0.0", &v));
    assert!(swift_matches("from:1.2.3", &v), "from is lower-inclusive");
    assert!(!swift_matches("from:2.0.0", &v));
    assert!(
        !swift_matches("from:1.3.0", &v),
        "upToNextMajor caps the major"
    );
    assert!(swift_matches("minor:1.2.0", &v));
    assert!(!swift_matches("minor:1.3.0", &v));
    assert!(swift_matches("range:1.0.0..<2.0.0", &v));
    assert!(swift_matches("range:1.2.3..<2.0.0", &v), "lower-inclusive");
    assert!(swift_matches("range:1.2.3..<2.0.0", &v));
    assert!(!swift_matches("range:1.2.4..<2.0.0", &v));
    assert!(!swift_matches("branch:main", &v));
    assert!(!swift_matches("revision:abc", &v));
    assert!(swift_matches("branch:main||from:1.0.0", &v), "alternatives");
}
