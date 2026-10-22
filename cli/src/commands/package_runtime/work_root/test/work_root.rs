//! Focused work root regression tests.
//! Test hồi quy tập trung cho work root.

use super::*;

#[test]
fn creates_workspace_under_configured_data_root() {
    let data = tempfile::tempdir().expect("isolated data root");
    let root = work_root_at(data.path(), PackageRunPurpose::Dlx).expect("safe dlx root");
    assert_eq!(
        root,
        data.path()
            .canonicalize()
            .expect("canonical data root")
            .join("cache/web/dlx")
    );
    let project = tempfile::Builder::new()
        .prefix(DLX_WORK_PREFIX)
        .tempdir_in(&root)
        .expect("fresh workspace");
    assert!(project.path().starts_with(&root));
}

#[test]
fn create_initializers_use_a_separate_mgc_managed_work_root() {
    let data = tempfile::tempdir().expect("isolated data root");
    let root = work_root_at(data.path(), PackageRunPurpose::CreateInitializer)
        .expect("safe create initializer root");
    assert_eq!(
        root,
        data.path()
            .canonicalize()
            .expect("canonical data root")
            .join("cache/web/create-initializers")
    );
}

#[test]
fn refuses_file_in_cache_directory_chain() {
    let data = tempfile::tempdir().expect("isolated data root");
    std::fs::write(data.path().join("cache"), "not a directory").expect("write obstruction");
    assert!(work_root_at(data.path(), PackageRunPurpose::Dlx).is_err());
}

#[cfg(unix)]
#[test]
fn refuses_symlinks_at_every_cache_directory_level() {
    for position in 0..DLX_WORK_DIRECTORY_COMPONENTS.len() {
        let data = tempfile::tempdir().expect("isolated data root");
        let outside = tempfile::tempdir().expect("isolated outside directory");
        let mut path = data.path().to_path_buf();
        for component in &DLX_WORK_DIRECTORY_COMPONENTS[..position] {
            path.push(component);
            std::fs::create_dir(&path).expect("create real parent");
        }
        path.push(DLX_WORK_DIRECTORY_COMPONENTS[position]);
        std::os::unix::fs::symlink(outside.path(), &path).expect("plant redirected cache");
        assert!(
            work_root_at(data.path(), PackageRunPurpose::Dlx).is_err(),
            "cache symlink must be rejected"
        );
        assert_eq!(
            std::fs::read_dir(outside.path())
                .expect("inspect outside directory")
                .count(),
            0
        );
    }
}
