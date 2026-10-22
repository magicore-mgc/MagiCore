// path_key tests — test cho path_key

use super::source_path_key;

#[cfg(unix)]
#[test]
fn source_path_key_distinguishes_invalid_utf8() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use std::path::Path;

    let path_a = Path::new(OsStr::from_bytes(b"source-\x80.ts"));
    let path_b = Path::new(OsStr::from_bytes(b"source-\x81.ts"));

    assert_ne!(source_path_key(path_a), source_path_key(path_b));
}
