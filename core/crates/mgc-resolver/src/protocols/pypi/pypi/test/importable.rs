//! Check wheel materialization safety and authenticated runtime contents.
//! Kiểm tra an toàn giải nén wheel và nội dung runtime đã xác thực.

use super::super::super::*;

use crate::protocols::pypi::wheel_record::test_support::{entry_for, stored_zip};

#[test]
fn registry_identity_and_artifact_name_cannot_escape_cache_paths() {
    assert_eq!(
        PypiProtocol::importable_site_dirname("six", "1.17.0")
            .expect("test operation must succeed"),
        "six-1.17.0"
    );
    for (name, version) in [
        ("../../outside", "1.0.0"),
        ("six", "1.0.0-../../outside"),
        ("six", r"1.0.0-..\outside"),
    ] {
        assert!(
            PypiProtocol::importable_site_dirname(name, version).is_err(),
            "must reject {name}@{version}"
        );
    }
    assert!(
        PypiProtocol::artifact_filename("https://files.pythonhosted.org/%2e%2e%2foutside.whl")
            .is_err()
    );
}

#[test]
fn native_runtime_accepts_only_pure_python_wheels() {
    assert!(
        PypiProtocol::is_importable_pure_wheel(
            "https://files.pythonhosted.org/six-1.17.0-py3-none-any.whl"
        )
        .expect("test operation must succeed")
    );
    assert!(
        !PypiProtocol::is_importable_pure_wheel(
            "https://files.pythonhosted.org/numpy-2.0.0-cp312-cp312-macosx_14_0_arm64.whl"
        )
        .expect("test operation must succeed")
    );
    assert!(
        !PypiProtocol::is_importable_pure_wheel(
            "https://files.pythonhosted.org/example-1.0.0.tar.gz"
        )
        .expect("test operation must succeed")
    );
}

#[test]
fn materializers_reject_unsafe_registry_paths_before_writing() {
    let dir = tempfile::tempdir().expect("test operation must succeed");
    let protocol = PypiProtocol::new("https://pypi.org");
    assert!(
        protocol
            .materialize(
                &entry_for("https://files.pythonhosted.org/%2e%2e%2foutside.whl"),
                b"payload",
                dir.path(),
            )
            .is_err()
    );
    let mut entry = entry_for("https://files.pythonhosted.org/x/six-1.17.0-py3-none-any.whl");
    entry.version = "1.0.0-../../outside".to_string();
    assert!(
        protocol
            .materialize_importable(&entry, b"not a zip", dir.path())
            .is_err()
    );
    assert!(
        std::fs::read_dir(dir.path())
            .expect("test operation must succeed")
            .next()
            .is_none()
    );
}

/// Pure-python wheels unpack into an importable site dir (RECORD
/// verified — a tampered member fails the unpack, not the import).
/// RECORD xác thực khiến member bị sửa thất bại lúc giải nén, không phải khi import.
#[test]
fn materialize_importable_unpacks_pure_wheel() {
    use base64::Engine;
    use sha2::{Digest, Sha256};
    let b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    let six_py = b"__version__ = '1.17.0'\n";
    let meta = b"Name: six\nVersion: 1.17.0\n";
    let record_body = format!(
        "six.py,sha256={},{}\nsix-1.17.0.dist-info/METADATA,sha256={},{}\nsix-1.17.0.dist-info/RECORD,,\n",
        b64.encode(Sha256::digest(six_py)),
        six_py.len(),
        b64.encode(Sha256::digest(meta)),
        meta.len(),
    );
    let wheel = stored_zip(&[
        ("six.py", six_py.as_slice()),
        ("six-1.17.0.dist-info/METADATA", meta.as_slice()),
        ("six-1.17.0.dist-info/RECORD", record_body.as_bytes()),
    ]);
    let dir = tempfile::tempdir().expect("test operation must succeed");
    let site = PypiProtocol::new("https://pypi.org")
        .materialize_importable(
            &entry_for("https://files.pythonhosted.org/x/six-1.17.0-py2.py3-none-any.whl"),
            &wheel,
            dir.path(),
        )
        .expect("test operation must succeed")
        .expect("pure wheel must unpack");
    assert!(site.join("six.py").exists(), "six.py importable");
}

/// Compiled wheels are NOT unpacked (ABI policy) — None, honestly.
/// Wheel biên dịch không được giải nén theo policy ABI; kết quả là None.
#[test]
fn materialize_importable_skips_compiled_wheel() {
    // Serialized against the tests that rewrite the process-wide variable.
    // (Tuần tự với các test thay đổi env toàn process.)
    let _guard = super::test_support::python_version_env_serial();
    let previous = std::env::var_os("MGC_PYTHON_VERSION");
    // SAFETY/AN TOÀN: the compatibility check must reach the ABI verdict, so
    // declare an explicit consumer for the duration of this test.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var("MGC_PYTHON_VERSION", "3.9");
    }
    let wheel = stored_zip(&[("x.so", b"fake")]);
    let dir = tempfile::tempdir().expect("test operation must succeed");
    let site = PypiProtocol::new("https://pypi.org").materialize_importable(
        &entry_for("https://files.pythonhosted.org/x/x-1.0-cp39-cp39-manylinux1_x86_64.whl"),
        &wheel,
        dir.path(),
    );
    // SAFETY/AN TOÀN: restore the process environment captured above.
    #[allow(unsafe_code)]
    unsafe {
        match &previous {
            Some(value) => std::env::set_var("MGC_PYTHON_VERSION", value),
            None => std::env::remove_var("MGC_PYTHON_VERSION"),
        }
    }
    let site = site.expect("test operation must succeed");
    assert!(site.is_none(), "compiled wheel must not unpack");
}

/// A compiled wheel without `MGC_PYTHON_VERSION` must name the artifact it
/// refused, so the user learns which dependency needs the interpreter minor
/// instead of a bare "cannot materialize".
/// (Wheel biên dịch thiếu `MGC_PYTHON_VERSION` phải nêu tên artifact bị từ chối.)
#[test]
fn compiled_wheel_without_declared_python_names_the_artifact() {
    // Serialized against the tests that rewrite the process-wide variable.
    // (Tuần tự với các test thay đổi env toàn process.)
    let _guard = super::test_support::python_version_env_serial();
    let previous = std::env::var_os("MGC_PYTHON_VERSION");
    // SAFETY: test-only env mutation, serialized and restored below.
    // AN TOÀN: chỉ test đổi env, tuần tự và phục hồi bên dưới.
    #[allow(unsafe_code)]
    unsafe {
        std::env::remove_var("MGC_PYTHON_VERSION");
    }
    let error = PypiProtocol::is_importable_runtime_wheel(
        "https://files.pythonhosted.org/j/jiter/jiter-0.18.0-cp314-cp314-macosx_11_0_arm64.whl",
    )
    .expect_err("a compiled wheel without a declared runtime must fail closed");
    // SAFETY/AN TOÀN: restore the process environment captured above.
    #[allow(unsafe_code)]
    unsafe {
        match previous {
            Some(value) => std::env::set_var("MGC_PYTHON_VERSION", value),
            None => std::env::remove_var("MGC_PYTHON_VERSION"),
        }
    }
    let rendered = error.to_string();
    assert!(rendered.contains("jiter-0.18.0-cp314"), "{rendered}");
    assert!(rendered.contains("MGC_PYTHON_VERSION"), "{rendered}");
}
