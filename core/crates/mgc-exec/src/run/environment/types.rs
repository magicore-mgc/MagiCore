//! Environment-owned temporary path type.
//! Kiểu đường dẫn tạm do owner môi trường sở hữu.

use std::path::PathBuf;

#[derive(Debug)]
pub(in crate::run) struct ShadowPath {
    pub(in crate::run::implementation) dir: PathBuf,
    pub(in crate::run::implementation) cleaned: bool,
    pub(in crate::run::implementation) cleanup_attempted: bool,
}
