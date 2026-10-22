//! Shared filesystem test fixture — fixture filesystem dùng chung trong test.
use std::fs;
use std::io::Write;

pub(super) fn write(root: &std::path::Path, rel: &str, content: &str) {
    let p = root.join(rel);
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    let mut f = fs::File::create(p).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}
