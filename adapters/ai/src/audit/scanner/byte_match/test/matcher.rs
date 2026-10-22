//! Unit cases for binary substring matching.
//! Test đơn vị cho so khớp chuỗi byte nhị phân.

use super::super::contains_bytes;

#[test]
fn test_contains_bytes() {
    assert!(contains_bytes(b"hello world", b"world"));
    assert!(!contains_bytes(b"hello world", b"foo"));
    assert!(contains_bytes(b"os.system", b"os.system"));
}
