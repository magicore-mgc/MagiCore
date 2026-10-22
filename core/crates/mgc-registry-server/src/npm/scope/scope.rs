//! Canonicalize scoped package names.
//! Chuẩn hóa tên package có scope.

pub(in crate::npm) fn scoped_full(scope: &str, name: &str) -> String {
    format!("@{}/{}", scope.trim_start_matches('@'), name)
}
