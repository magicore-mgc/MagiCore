//! Cow Clone source regression test.
//! Test hồi quy source cho owner cow clone.

use std::fs;

#[test]
fn clone_succeeds_or_leaves_a_clean_fallback_path() {
    let temp = tempfile::tempdir().unwrap();
    let src = temp.path().join("source");
    let dest = temp.path().join("clone");
    fs::write(&src, b"cow payload").unwrap();

    let cloned = super::super::cow_clone::try_cow_clone(&src, &dest);

    match cloned {
        Some(()) => assert_eq!(fs::read(&dest).unwrap(), b"cow payload"),
        None => assert!(!dest.exists()),
    }
}
