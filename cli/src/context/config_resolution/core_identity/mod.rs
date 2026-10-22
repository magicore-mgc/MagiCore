//! Own canonical core comparison and project-name derivation.
//! Sở hữu so sánh core chuẩn hóa và tạo tên project từ đường dẫn.

mod core_identity;

pub(super) use core_identity::{ensure_override_matches_identity, project_name};
