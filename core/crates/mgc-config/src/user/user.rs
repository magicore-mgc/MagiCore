//! User-level MagiCore configuration model.
//! Model cấu hình MagiCore cấp user.
/// User-level configuration
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserConfig {
    pub name: Option<String>,
    pub email: Option<String>,
}
