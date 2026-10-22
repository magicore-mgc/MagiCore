//! Tests temp naming and conservative owner-pid parsing.
//! Kiểm tra tên temp và cách đọc pid thận trọng.

use super::super::{temp_name, temp_pid};

#[test]
fn generated_name_contains_current_pid_and_parseable_nonce() {
    let name = temp_name();
    assert!(name.starts_with(&format!("mgc.lock.tmp.{}.", std::process::id())));
    assert_eq!(temp_pid(&name), Some(std::process::id()));
}

#[test]
fn malformed_temp_names_have_no_owner_pid() {
    assert_eq!(temp_pid("other.tmp.12.x"), None);
    assert_eq!(temp_pid("mgc.lock.tmp.not-a-pid.x"), None);
    assert_eq!(temp_pid("mgc.lock.tmp."), None);
}
