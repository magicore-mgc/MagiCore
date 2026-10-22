//! Project detector capability implementation.
//! Triển khai capability nhận diện dự án.

use super::super::CoreIdent;
use std::path::Path;

/// Detects whether a project root belongs to this core.
/// Method thật của core (`can_handle`) — di chuyển nguyên vẹn.
pub trait ProjectDetector: CoreIdent {
    fn can_handle(&self, project_root: &Path) -> bool;
}
