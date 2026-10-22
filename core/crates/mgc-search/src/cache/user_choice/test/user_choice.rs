//! Tests for learned registry choice SQL operations.
//! Test thao tác SQL của lựa chọn registry đã học.

use rusqlite::Connection;

use crate::types::Registry;

use super::{get_user_choice, track_choice};

#[test]
fn choice_is_hidden_until_the_three_install_threshold() {
    let db = Connection::open_in_memory().expect("registry-choice SQL operation succeeds");
    super::super::super::schema::initialize(&db).expect("registry-choice SQL operation succeeds");

    track_choice(&db, "gin", Registry::Go).expect("registry-choice SQL operation succeeds");
    track_choice(&db, "gin", Registry::Go).expect("registry-choice SQL operation succeeds");
    assert!(
        get_user_choice(&db, "gin")
            .expect("registry-choice SQL operation succeeds")
            .is_none()
    );

    track_choice(&db, "gin", Registry::Go).expect("registry-choice SQL operation succeeds");
    assert_eq!(
        get_user_choice(&db, "gin").expect("registry-choice SQL operation succeeds"),
        Some(Registry::Go)
    );
}

#[test]
fn registry_with_higher_count_is_preferred() {
    let db = Connection::open_in_memory().expect("registry-choice SQL operation succeeds");
    super::super::super::schema::initialize(&db).expect("registry-choice SQL operation succeeds");

    track_choice(&db, "cors", Registry::Npm).expect("registry-choice SQL operation succeeds");
    track_choice(&db, "cors", Registry::Npm).expect("registry-choice SQL operation succeeds");
    for _ in 0..3 {
        track_choice(&db, "cors", Registry::Go).expect("registry-choice SQL operation succeeds");
    }

    assert_eq!(
        get_user_choice(&db, "cors").expect("registry-choice SQL operation succeeds"),
        Some(Registry::Go)
    );
}
