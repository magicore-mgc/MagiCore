//! Verify PyPI coordinator construction and version constraints.
//! Kiểm tra dựng coordinator PyPI và ràng buộc version.

use super::super::super::*;
use mgc_types::Version;

#[test]
fn wheel_identity_accepts_pep440_equivalent_trailing_zero_release() {
    let metadata = b"Metadata-Version: 2.4\nName: packaging\nVersion: 26.3\n\n";
    assert!(verify_wheel_identity(metadata, "packaging", "26.3.0").is_ok());
}

#[test]
fn wheel_identity_rejects_unsupported_or_different_versions() {
    let equivalent = b"Metadata-Version: 2.4\nName: packaging\nVersion: 26.3.0.0\n\n";
    assert!(verify_wheel_identity(equivalent, "packaging", "26.3.0").is_ok());

    for actual in ["26.3.1", "26.3rc1", "26.3.post1", "26.3+vendor", "26..3"] {
        let metadata = format!("Metadata-Version: 2.4\nName: packaging\nVersion: {actual}\n\n");
        assert!(
            verify_wheel_identity(metadata.as_bytes(), "packaging", "26.3.0").is_err(),
            "accepted {actual}"
        );
    }
}

#[test]
fn pep440_compatible_release() {
    let v = Version::parse("1.4.2").expect("test operation must succeed");
    assert!(pep440_matches("~=1.4", &v));
    assert!(!pep440_matches(
        "~=1.4",
        &Version::parse("2.0.0").expect("test operation must succeed")
    ));
    assert!(pep440_matches(
        "~=1.4.5",
        &Version::parse("1.4.9").expect("test operation must succeed")
    ));
    assert!(!pep440_matches(
        "~=1.4.5",
        &Version::parse("1.5.0").expect("test operation must succeed")
    ));
}

#[test]
fn pep440_eq_wildcard() {
    assert!(pep440_matches(
        "==1.2.*",
        &Version::parse("1.2.9").expect("test operation must succeed")
    ));
    assert!(!pep440_matches(
        "==1.2.*",
        &Version::parse("1.3.0").expect("test operation must succeed")
    ));
    assert!(pep440_matches(
        "!=1.2",
        &Version::parse("1.3.0").expect("test operation must succeed")
    ));
    assert!(!pep440_matches(
        "!=1.2",
        &Version::parse("1.2.0").expect("test operation must succeed")
    ));
}

#[test]
fn pep508_parsing() {
    let (name, range, marker) =
        parse_pep508("requests>=2.0 ; extra == \"security\"").expect("test operation must succeed");
    assert_eq!(name, "requests");
    assert_eq!(range, ">=2.0");
    assert!(
        marker
            .expect("test operation must succeed")
            .contains("extra")
    );
    let (name, range, marker) = parse_pep508("idna").expect("test operation must succeed");
    assert_eq!(name, "idna");
    assert_eq!(range, "*");
    assert!(marker.is_none());
}

#[test]
fn wheel_tags_last_three() {
    assert_eq!(
        wheel_tags("numpy-1.26.4-cp312-cp312-macosx_11_0_arm64.whl"),
        (
            "cp312".to_string(),
            "cp312".to_string(),
            "macosx_11_0_arm64".to_string()
        )
    );
    assert_eq!(
        wheel_tags("pkg-1.0-py3-none-any.whl"),
        ("py3".to_string(), "none".to_string(), "any".to_string())
    );
}

#[test]
fn abi_compatible_cases() {
    // Universal + exact + abi3 floors.
    // Phủ trường hợp universal, khớp chính xác và mức nền ABI3.
    assert!(abi_compatible("py3", "none", 3, 9));
    assert!(abi_compatible("cp39", "cp39", 3, 9));
    assert!(!abi_compatible("cp310", "cp310", 3, 9));
    assert!(abi_compatible("cp310", "cp310", 3, 10));
    assert!(abi_compatible("cp37", "abi3", 3, 9));
    assert!(abi_compatible("cp39", "abi3", 3, 9));
    assert!(!abi_compatible("cp310", "abi3", 3, 9));
    assert!(!abi_compatible("cp39", "cp39", 3, 10));
    assert!(abi_compatible("py2.py3", "none", 3, 9));
}

#[test]
fn requires_python_allows_cases() {
    assert!(requires_python_allows(">=3.9", 3, 9));
    assert!(!requires_python_allows(">=3.10", 3, 9));
    assert!(requires_python_allows(">=3.9,<4", 3, 9));
    assert!(!requires_python_allows(">=3.9,<3.10", 3, 10));
    assert!(requires_python_allows("==3.9.*", 3, 9));
    assert!(!requires_python_allows("==3.9.*", 3, 10));
    assert!(requires_python_allows("~=3.9", 3, 9));
    assert!(requires_python_allows("~=3.9", 3, 12));
    assert!(!requires_python_allows("~=3.9", 4, 0));
    assert!(!requires_python_allows("!=3.9.*", 3, 9));
    assert!(requires_python_allows("!=3.9.*", 3, 10));
    assert!(!requires_python_allows("bogus", 3, 9));
}

#[test]
fn consumer_python_reads_env() {
    // Serialized: env is process-global and tests run in threads.
    // (Tuần tự hóa: env toàn process.)
    static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = SERIAL.lock().expect("test operation must succeed");
    let old = std::env::var("MGC_PYTHON_VERSION").ok();
    // SAFETY: test-only, held SERIAL, restored below before release.
    // AN TOÀN: chỉ test, giữ khóa SERIAL và khôi phục trước khi nhả khóa.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var("MGC_PYTHON_VERSION", "3.9");
        assert_eq!(consumer_python(), Some((3, 9)));
        let incompatible_file = PypiFile {
            filename: "demo-1.0.0-py3-none-any.whl".to_string(),
            url: "https://files.pythonhosted.org/demo.whl".to_string(),
            digests: PypiDigests::default(),
            packagetype: "bdist_wheel".to_string(),
            requires_python: Some(">=3.10".to_string()),
            upload_time_iso_8601: None,
        };
        assert!(
            select_file(std::slice::from_ref(&incompatible_file), consumer_python()).is_none(),
            "a wheel-specific Requires-Python constraint must reject this artifact"
        );
        let compatible_file = PypiFile {
            requires_python: Some(">=3.9".to_string()),
            ..incompatible_file
        };
        assert_eq!(
            select_file(std::slice::from_ref(&compatible_file), consumer_python())
                .map(|file| file.filename.as_str()),
            Some(compatible_file.filename.as_str())
        );
        std::env::set_var("MGC_PYTHON_VERSION", "310");
        assert_eq!(consumer_python(), Some((3, 10)));
        std::env::set_var("MGC_PYTHON_VERSION", "39");
        assert_eq!(consumer_python(), Some((3, 9)));
        std::env::remove_var("MGC_PYTHON_VERSION");
        assert_eq!(consumer_python(), None);
        if let Some(v) = old {
            std::env::set_var("MGC_PYTHON_VERSION", v);
        }
    }
}
