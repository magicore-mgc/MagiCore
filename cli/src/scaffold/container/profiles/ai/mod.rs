//! AI Python development container.
//! Container phát triển AI Python do MGC quản lý.

use crate::scaffold::container::error;
#[path = "ai.rs"]
mod ai;
pub(in crate::scaffold::container) use ai::write_ai_docker_assets;
