//! Keg store tests: immutable create, no-overwrite, list, remove.
//! Test store keg: tạo bất biến, không ghi đè, liệt kê, xoá.

use super::super::error::KegError;
use super::super::keg::{KEG_RECORD_FILE, KegStore, read_record};
use super::super::layout::{KegLayout, validate_library_name, validate_relative, validate_version};
use super::support::{Fixture, mode_of, write_tree};

#[test]
fn installed_keg_is_write_protected_recursively() {
    let fixture = Fixture::new();
    let keg = fixture.install("libcxx", "1.2.3");

    assert_eq!(mode_of(&keg) & 0o222, 0, "keg root must be read-only");
    assert_eq!(
        mode_of(&keg.join("shared").join("libcxx.txt")) & 0o222,
        0,
        "nested file must be read-only"
    );
    assert_eq!(
        mode_of(&keg.join("include")) & 0o222,
        0,
        "nested directory must be read-only"
    );
}

#[test]
fn create_keg_refuses_to_overwrite_an_existing_version() {
    let fixture = Fixture::new();
    fixture.install("libcxx", "1.2.3");
    let keg = fixture
        .layout
        .keg_dir("libcxx", &validate_version("1.2.3").unwrap());
    let before = std::fs::read(keg.join("README")).unwrap();

    let error = fixture
        .store
        .create_keg("libcxx", "1.2.3", |staging| {
            std::fs::write(staging.join("README"), b"destroyed").unwrap();
            Ok(())
        })
        .expect_err("reinstalling the same version must fail closed");

    assert!(matches!(error, KegError::KegExists { .. }), "{error:?}");
    assert_eq!(
        std::fs::read(keg.join("README")).unwrap(),
        before,
        "existing keg content must be untouched"
    );
}

#[test]
fn list_kegs_returns_every_version_oldest_first() {
    let fixture = Fixture::new();
    fixture.install("libcxx", "2.0.0");
    fixture.install("libcxx", "1.10.0");
    fixture.install("libcxx", "1.9.0");

    let records = fixture.store.list_kegs("libcxx").unwrap();
    let versions: Vec<&str> = records
        .iter()
        .map(|record| record.version.as_str())
        .collect();
    assert_eq!(versions, vec!["1.9.0", "1.10.0", "2.0.0"]);
}

#[test]
fn list_kegs_of_unknown_library_is_empty_not_an_error() {
    let fixture = Fixture::new();
    assert!(
        fixture
            .store
            .list_kegs("never-installed")
            .unwrap()
            .is_empty()
    );
}

#[test]
fn record_roundtrips_and_matches_sealed_content() {
    let fixture = Fixture::new();
    let keg = fixture.install("libcxx", "1.0.0");
    let record = read_record(&keg).unwrap();

    assert_eq!(record.name, "libcxx");
    assert_eq!(record.version, "1.0.0");
    assert_eq!(
        record.file_count, 3,
        "two files + one symlink record: {record:?}"
    );
    assert!(!record.tree_digest.is_empty());
    assert!(keg.join(KEG_RECORD_FILE).is_file());
    assert_eq!(fixture.store.open_keg("libcxx", "1.0.0").unwrap(), record);
}

#[test]
fn open_keg_reports_missing_version() {
    let fixture = Fixture::new();
    let error = fixture.store.open_keg("libcxx", "9.9.9").unwrap_err();
    assert!(matches!(error, KegError::KegNotFound { .. }), "{error:?}");
}

#[test]
fn explicit_remove_deletes_only_the_requested_version() {
    let fixture = Fixture::new();
    fixture.install("libcxx", "1.0.0");
    fixture.install("libcxx", "1.1.0");

    fixture.store.remove_keg("libcxx", "1.0.0").unwrap();

    let versions: Vec<String> = fixture
        .store
        .list_kegs("libcxx")
        .unwrap()
        .into_iter()
        .map(|record| record.version)
        .collect();
    assert_eq!(versions, vec!["1.1.0"]);
}

#[test]
fn staging_directories_are_never_listed_as_kegs() {
    let fixture = Fixture::new();
    fixture.install("libcxx", "1.0.0");
    let stale = fixture
        .layout
        .cellar_root()
        .join("libcxx")
        .join(format!("{}1.0.1.dead.0", super::super::keg::STAGING_PREFIX));
    std::fs::create_dir(&stale).unwrap();

    let versions: Vec<String> = fixture
        .store
        .list_kegs("libcxx")
        .unwrap()
        .into_iter()
        .map(|record| record.version)
        .collect();
    assert_eq!(versions, vec!["1.0.0"]);
}

#[test]
fn failed_materialize_leaves_no_keg_and_no_staging() {
    let fixture = Fixture::new();
    let error = fixture
        .store
        .create_keg("libcxx", "3.0.0", |staging| {
            std::fs::write(staging.join("partial"), b"x").unwrap();
            Err(KegError::Layout("materialize failed".to_string()))
        })
        .unwrap_err();

    assert!(matches!(error, KegError::Layout(_)));
    assert!(
        !fixture
            .layout
            .keg_dir("libcxx", &validate_version("3.0.0").unwrap())
            .exists()
    );
    let leftovers: Vec<_> = std::fs::read_dir(fixture.layout.cellar_root().join("libcxx"))
        .unwrap()
        .flatten()
        .collect();
    assert!(
        leftovers.is_empty(),
        "staging must be cleaned: {leftovers:?}"
    );
}

#[test]
fn library_names_and_versions_are_validated_before_touching_disk() {
    let fixture = Fixture::new();
    for unsafe_name in ["", "..", "../escape", "a/b", "a\\b", "nul\0"] {
        assert!(
            validate_library_name(unsafe_name).is_err(),
            "must reject {unsafe_name:?}"
        );
        assert!(matches!(
            fixture.store.create_keg(unsafe_name, "1.0.0", |_| Ok(())),
            Err(KegError::UnsafePath { .. })
        ));
    }
    for bad_version in ["", "1", "1.0.0+build", "latest", "../1.0.0"] {
        assert!(
            validate_version(bad_version).is_err(),
            "must reject {bad_version:?}"
        );
    }
    assert!(validate_relative(std::path::Path::new("../x")).is_err());
    assert!(validate_relative(std::path::Path::new("a/../../x")).is_err());
    assert!(validate_relative(std::path::Path::new("a/b")).is_ok());
}

#[test]
fn store_can_be_rebuilt_from_the_same_layout() {
    let fixture = Fixture::new();
    fixture.install("libcxx", "1.0.0");
    let reopened = KegStore::new(KegLayout::with_data_root(fixture.data_root()));
    assert_eq!(reopened.list_kegs("libcxx").unwrap().len(), 1);
    let _ = write_tree;
}
