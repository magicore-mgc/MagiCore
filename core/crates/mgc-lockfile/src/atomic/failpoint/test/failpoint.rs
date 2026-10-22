//! Checks the public ordered failpoint contract without changing process environment.
//! Kiểm tra contract phase công khai mà không sửa environment của process.

use super::super::LOCK_FAILPOINTS;

#[test]
fn lock_failpoint_names_preserve_transaction_order() {
    assert_eq!(
        LOCK_FAILPOINTS,
        &[
            "lock-before-temp-write",
            "lock-after-temp-fsync",
            "lock-before-rename",
            "lock-after-rename",
            "lock-after-dir-fsync",
        ]
    );
}
