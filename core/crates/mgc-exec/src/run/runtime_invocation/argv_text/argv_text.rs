//! Read one argv item only when it exists and is valid UTF-8.
//! Chỉ đọc argv item khi tồn tại và là UTF-8 hợp lệ.

use std::ffi::OsString;

pub(in crate::run::implementation) fn argv_text(argv: &[OsString], index: usize) -> Option<&str> {
    argv.get(index)?.to_str()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
