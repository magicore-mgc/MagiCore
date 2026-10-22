//! Compares paths by component-prefix relationship for Store safety checks.
//! So sánh path theo quan hệ tiền tố component cho kiểm tra an toàn Store.

use std::path::Path;

pub(in crate::commands) fn paths_overlap(left: &Path, right: &Path) -> bool {
    left.starts_with(right) || right.starts_with(left)
}
