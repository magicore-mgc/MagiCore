use super::super::*;

#[test]
fn rust_package_projection_rejects_path_like_locked_names() {
    let mut package = locked_package("fixture", &[]);
    package.name = "../outside".to_string();
    let lock = Lockfile {
        packages: vec![package],
        ..Lockfile::new()
    };
    let error = rust_packages(&lock).unwrap_err();
    assert!(error.to_string().contains("invalid locked crate name"));
}

#[test]
fn rust_package_projection_accepts_known_edges_but_rejects_unknown_markers() {
    let mut package = locked_package("itoa", &[]);
    package.markers = Some(vec!["optional:no-panic".to_string()]);
    let lock = Lockfile {
        packages: vec![package.clone()],
        ..Lockfile::new()
    };
    assert_eq!(rust_packages(&lock).unwrap().len(), 1);

    package.markers = Some(vec!["target:cfg(any())".to_string()]);
    let lock = Lockfile {
        packages: vec![package.clone()],
        ..Lockfile::new()
    };
    assert_eq!(rust_packages(&lock).unwrap().len(), 1);

    package.markers = Some(vec!["target:cfg(windows)".to_string()]);
    let lock = Lockfile {
        packages: vec![package],
        ..Lockfile::new()
    };
    assert_eq!(rust_packages(&lock).unwrap().len(), 1);

    let mut package = locked_package("itoa", &[]);
    package.markers = Some(vec!["unknown:marker".to_string()]);
    let lock = Lockfile {
        packages: vec![package],
        ..Lockfile::new()
    };
    let error = rust_packages(&lock).unwrap_err();
    assert!(error.to_string().contains("unknown:marker"));
}
