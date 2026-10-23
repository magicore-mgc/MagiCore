//! Builds a known-valid version value for focused tests.
//! Tạo Version hợp lệ dùng trong test chuyên biệt.

use super::super::Version;

pub(in crate::version) fn v(input: &str) -> Version {
    Version::parse(input).expect("test fixture version must be valid")
}
