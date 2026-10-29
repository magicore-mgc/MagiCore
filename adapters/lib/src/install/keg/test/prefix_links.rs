//! Prefix link tests: relative targets, atomic replace, managed removal.
//! Test link prefix: đích tương đối, thay atomic, xoá có kiểm soát.

use std::path::{Path, PathBuf};

use super::super::error::KegError;
use super::super::layout::validate_version;
use super::super::prefix::{
    create_symlink, link_target, place_link, plan_links, relative_target, remove_link,
};
use super::support::Fixture;

#[test]
fn relative_target_is_computed_from_the_link_parent() {
    let target = relative_target(
        Path::new("/d/lib/opt/pkg"),
        Path::new("/d/lib/Cellar/pkg/1.0.0"),
    )
    .unwrap();
    assert_eq!(target, PathBuf::from("../../Cellar/pkg/1.0.0"));

    let nested = relative_target(
        Path::new("/d/lib/prefix/pkg"),
        Path::new("/d/lib/Cellar/pkg/1.0.0"),
    )
    .unwrap();
    assert_eq!(nested, PathBuf::from("../../Cellar/pkg/1.0.0"));
}

#[test]
fn relative_target_refuses_unrelated_roots() {
    let error = relative_target(Path::new("/a/x"), Path::new("/a/x")).unwrap_err();
    assert!(matches!(error, KegError::UnsafePath { .. }), "{error:?}");

    // Relative links cannot leave `/`, so they must not traverse a `..` step
    // that would land outside the shared root (guarded in verify_keg).
    // (Link tương đối không thể vượt `/`; bước `..` vượt root bị chặn ở verify_keg.)
    assert_eq!(
        relative_target(Path::new("/a/x"), Path::new("/b/y")).unwrap(),
        PathBuf::from("../../b/y")
    );
    assert!(relative_target(Path::new("rel/x"), Path::new("/abs/y")).is_err());
}

#[test]
fn planned_links_point_at_the_keg_and_stay_relative() {
    let fixture = Fixture::new();
    let version = validate_version("1.4.0").unwrap();
    let keg = fixture.layout.keg_dir("libcxx", &version);
    let links = plan_links(&fixture.layout, "libcxx", &version, &keg).unwrap();

    assert_eq!(links.len(), 2);
    for link in &links {
        assert!(link.target.is_relative(), "{:?}", link.target);
        assert!(link.path.starts_with(fixture.data_root()));
    }
}

#[test]
fn place_link_replaces_atomically_and_links_resolve_to_content() {
    let fixture = Fixture::new();
    let version = validate_version("1.0.0").unwrap();
    let keg = fixture.install("libcxx", "1.0.0");
    let links = plan_links(&fixture.layout, "libcxx", &version, &keg).unwrap();
    let current = fixture
        .layout
        .opt_dir("libcxx")
        .join(super::super::layout::ACTIVE_LINK);

    for _ in 0..2 {
        for link in &links {
            if link.path == current {
                place_link(&link.path, &link.target).unwrap();
            } else {
                let parent = link.path.parent().unwrap();
                std::fs::create_dir_all(parent).unwrap();
                place_link(&link.path, &link.target).unwrap();
            }
        }
    }
    // Both links resolve to the keg dir and see its content.
    // (Cả hai link resolve đúng keg và thấy nội dung.)
    assert_eq!(
        std::fs::canonicalize(&current).unwrap(),
        std::fs::canonicalize(&keg).unwrap()
    );
    assert_eq!(
        std::fs::read(current.join("README")).unwrap(),
        b"README 1.0.0"
    );
    let prefix = fixture.layout.version_prefix_dir("libcxx", &version);
    assert_eq!(
        std::fs::read(prefix.join("README")).unwrap(),
        b"README 1.0.0"
    );
}

#[test]
fn link_target_reads_back_the_relative_target() {
    let fixture = Fixture::new();
    let link = fixture.data_root().join("probe-link");
    std::fs::create_dir_all(fixture.data_root()).unwrap();
    create_symlink(Path::new("Cellar/pkg/1.0.0"), &link).unwrap();
    assert_eq!(
        link_target(&link).unwrap(),
        Some(PathBuf::from("Cellar/pkg/1.0.0"))
    );
    assert_eq!(
        link_target(&fixture.data_root().join("absent")).unwrap(),
        None
    );
}

#[test]
fn remove_link_refuses_to_delete_a_real_directory() {
    let fixture = Fixture::new();
    let directory = fixture.data_root().join("real-dir");
    std::fs::create_dir_all(&directory).unwrap();
    let error = remove_link(&directory).unwrap_err();
    assert!(matches!(error, KegError::UnsafePath { .. }), "{error:?}");
    assert!(directory.is_dir(), "the real directory must survive");
    remove_link(&fixture.data_root().join("absent")).unwrap();
}
