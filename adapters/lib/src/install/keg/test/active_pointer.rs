//! Active pointer tests: atomic write, read-back, corruption is loud.
//! Test con trỏ active: ghi atomic, đọc lại, hỏng phải báo lỗi.

use super::super::active::{ActiveState, pointer_path, read_active, write_active};
use super::super::error::KegError;
use super::support::Fixture;

#[test]
fn missing_pointer_reads_as_none() {
    let fixture = Fixture::new();
    assert_eq!(read_active(&fixture.layout, "libcxx").unwrap(), None);
}

#[test]
fn pointer_roundtrips_with_previous_version() {
    let fixture = Fixture::new();
    let state = ActiveState {
        version: "2.0.0".to_string(),
        previous: Some("1.0.0".to_string()),
    };
    write_active(&fixture.layout, "libcxx", &state).unwrap();
    assert_eq!(read_active(&fixture.layout, "libcxx").unwrap(), Some(state));
}

#[test]
fn pointer_write_replaces_without_leaving_staging_files() {
    let fixture = Fixture::new();
    write_active(&fixture.layout, "libcxx", &ActiveState::initial("1.0.0")).unwrap();
    write_active(&fixture.layout, "libcxx", &ActiveState::initial("1.1.0")).unwrap();

    let opt = fixture.layout.opt_dir("libcxx");
    let leftovers: Vec<String> = std::fs::read_dir(&opt)
        .unwrap()
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".tmp"))
        .collect();
    assert!(leftovers.is_empty(), "staging leftovers: {leftovers:?}");
    assert_eq!(
        read_active(&fixture.layout, "libcxx")
            .unwrap()
            .unwrap()
            .version,
        "1.1.0"
    );
}

#[test]
fn corrupt_pointer_is_an_error_not_a_silent_default() {
    let fixture = Fixture::new();
    let path = pointer_path(&fixture.layout, "libcxx");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, b"{not json").unwrap();

    let error = read_active(&fixture.layout, "libcxx").unwrap_err();
    assert!(matches!(error, KegError::InvalidActive { .. }), "{error:?}");
}

#[test]
fn empty_active_version_is_rejected() {
    let fixture = Fixture::new();
    let path = pointer_path(&fixture.layout, "libcxx");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, br#"{"version":""}"#).unwrap();

    let error = read_active(&fixture.layout, "libcxx").unwrap_err();
    assert!(matches!(error, KegError::InvalidActive { .. }), "{error:?}");
}
