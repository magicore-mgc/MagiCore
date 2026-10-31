//! Verify PyPI coordinator construction and version constraints.
//! Kiểm tra dựng coordinator PyPI và ràng buộc version.

use super::super::super::*;

/// GIL-flavor shorthand kept for the original ABI matrix; flavor-specific
/// behavior has its own cases below.
/// (Bí danh flavor GIL cho ma trận ABI gốc; flavor riêng có case bên dưới.)
fn abi_compatible(py: &str, abi: &str, major: u64, minor: u64) -> bool {
    abi_compatible_for_interpreter(py, abi, major, minor, InterpreterAbiFlavor::Gil)
}

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
fn wheel_identity_accepts_peppercorn_style_prereleases() {
    // Real PyPI metadata for httpx `1.0.0.dev6` (wheel and `pyproject.toml`
    // spell the same release differently). A release-only comparison rejected
    // this locked version with "identity does not match locked httpx@1.0.0.dev6".
    // (METADATA thật của PyPI: wheel và pyproject ghi cùng release khác nhau.
    // So sánh chỉ phần release từng từ chối nhầm httpx@1.0.0.dev6.)
    for metadata_version in ["1.0.dev6", "1.0.0.dev6", "1.0.0-dev.6", "v1.0.0.dev6"] {
        let metadata =
            format!("Metadata-Version: 2.5\nName: httpx\nVersion: {metadata_version}\n\n");
        assert!(
            verify_wheel_identity(metadata.as_bytes(), "httpx", "1.0.0.dev6").is_ok(),
            "wheel declared {metadata_version}"
        );
    }
    // A different pre-release number or a different release is still refused.
    // (Khác số pre-release hoặc khác release vẫn bị từ chối.)
    for metadata_version in ["1.0.dev7", "1.1.dev6", "1.0.0", "1.0.0.post1"] {
        let metadata =
            format!("Metadata-Version: 2.5\nName: httpx\nVersion: {metadata_version}\n\n");
        assert!(
            verify_wheel_identity(metadata.as_bytes(), "httpx", "1.0.0.dev6").is_err(),
            "accepted {metadata_version}"
        );
    }
}

#[test]
fn wheel_identity_normalizes_pep503_names_and_folded_headers() {
    // `zope.interface` is published as `zope-interface`; an underscore or dot
    // in the METADATA Name must not read as a different distribution.
    // (Tên PEP 503: `zope.interface` và `zope-interface` là cùng một package.)
    assert!(
        verify_wheel_identity(
            b"Metadata-Version: 2.4\nName: zope.interface\nVersion: 6.1\n\n",
            "zope-interface",
            "6.1"
        )
        .is_ok()
    );
    // Folded header continuation is part of the value (RFC 822).
    // (Header gấp dòng là một phần giá trị.)
    assert!(
        verify_wheel_identity(
            b"Metadata-Version: 2.4\nname: demo\nversion: 1.0\n .0\n\n",
            "demo",
            "1.0.0"
        )
        .is_ok()
    );
    // A second identity for the same field is ambiguous, never last-wins.
    // (Trường identity lặp lại là mơ hồ, không lấy giá trị cuối.)
    assert!(
        verify_wheel_identity(
            b"Metadata-Version: 2.4\nName: demo\nName: other\nVersion: 1.0\n\n",
            "demo",
            "1.0"
        )
        .is_err()
    );
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
fn abi_compatible_separates_free_threaded_builds() {
    // A `t`-suffixed interpreter only loads `t` binaries, and the GIL build of
    // the same version never loads them; mixing the two is a loader failure,
    // not a compatibility ambiguity.
    // (Bản `t` chỉ nạp binary `t`; bản GIL cùng version không nạp được chúng.)
    let gil = InterpreterAbiFlavor::Gil;
    let freethreaded = InterpreterAbiFlavor::FreeThreaded;
    assert!(abi_compatible_for_interpreter("cp314", "cp314", 3, 14, gil));
    assert!(abi_compatible_for_interpreter(
        "cp314t",
        "cp314t",
        3,
        14,
        freethreaded
    ));
    assert!(!abi_compatible_for_interpreter(
        "cp314t", "cp314t", 3, 14, gil
    ));
    assert!(!abi_compatible_for_interpreter(
        "cp314",
        "cp314",
        3,
        14,
        freethreaded
    ));
    // abi3 is a GIL-ABI contract.
    // (abi3 thuộc ABI GIL.)
    assert!(!abi_compatible_for_interpreter(
        "cp312",
        "abi3",
        3,
        14,
        freethreaded
    ));
    assert!(abi_compatible_for_interpreter("cp312", "abi3", 3, 14, gil));
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
    let _guard = super::test_support::python_version_env_serial();
    let old = std::env::var("MGC_PYTHON_VERSION").ok();
    // SAFETY: test-only, holds the shared lock, restored below before release.
    // AN TOÀN: chỉ test, giữ khóa SERIAL và khôi phục trước khi nhả khóa.
    #[allow(unsafe_code)]
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
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

#[test]
fn select_file_prefers_the_wheel_the_declared_interpreter_can_load() {
    // Every candidate shares the host platform, so only the `(py, abi)` pair
    // can separate them. A 3.12 consumer must never be handed the `cp314`
    // binary just because it is listed first, and a 3.14 consumer must take
    // the exact `cp314` wheel over the older `cp312` one.
    // (Mọi candidate cùng platform, chỉ `(py, abi)` phân biệt được. Consumer
    // 3.12 không được nhận binary `cp314`; consumer 3.14 lấy đúng wheel `cp314`.)
    let platform = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => "macosx_11_0_arm64",
        ("linux", "x86_64") => "manylinux_2_17_x86_64",
        _ => return,
    };
    let file = |stem: &str| PypiFile {
        filename: format!("{stem}-{platform}.whl"),
        url: format!("https://files.pythonhosted.org/{stem}.whl"),
        digests: PypiDigests::default(),
        packagetype: "bdist_wheel".to_string(),
        requires_python: Some(">=3.10".to_string()),
        upload_time_iso_8601: None,
    };
    let files = vec![
        file("demo-1.0.0-cp314-cp314"),
        file("demo-1.0.0-cp312-cp312"),
    ];
    assert_eq!(
        select_file(&files, Some((3, 12))).map(|chosen| chosen.filename.as_str()),
        Some(files[1].filename.as_str())
    );
    assert_eq!(
        select_file(&files, Some((3, 14))).map(|chosen| chosen.filename.as_str()),
        Some(files[0].filename.as_str())
    );
    // A free-threaded interpreter only loads the `t` wheel.
    // (Interpreter free-threaded chỉ nạp wheel có hậu tố `t`.)
    let threaded = vec![file("demo-1.0.0-cp314t-cp314t")];
    assert!(
        select_file(&threaded, Some((3, 14))).is_none(),
        "a GIL 3.14 interpreter cannot load a cp314t binary"
    );
}
