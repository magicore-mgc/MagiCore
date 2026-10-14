//! Regression tests for host-independent model artifact identities.
//! Regression cho định danh model artifact không phụ thuộc định dạng đường dẫn máy.

use super::{model_artifact_file_label, model_artifact_package_name};

#[test]
fn package_name_accepts_absolute_windows_paths_and_is_path_specific() {
    let first_path = r"C:\Users\RUNNER~1\AppData\Local\Temp\project\checkpoints\fixture.pth";
    let first = model_artifact_package_name(first_path).unwrap();
    let second = model_artifact_package_name(
        r"C:\Users\RUNNER~1\AppData\Local\Temp\project\checkpoints\other.pth",
    )
    .unwrap();

    assert!(first.as_str().starts_with("model-artifact-"));
    assert!(!first.as_str().contains(':'));
    assert!(!first.as_str().contains('\\'));
    assert_ne!(first, second);
    assert_eq!(model_artifact_file_label(first_path), "fixture.pth");
}

#[test]
fn audit_label_hides_absolute_host_paths_on_unix_and_windows() {
    assert_eq!(
        model_artifact_file_label("/Users/alice/project/checkpoints/fixture.pth"),
        "fixture.pth"
    );
    assert_eq!(
        model_artifact_file_label(r"C:\Users\alice\project\checkpoints\fixture.pth"),
        "fixture.pth"
    );
}
