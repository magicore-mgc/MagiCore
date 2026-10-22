//! Cloud CDK validation container.
//! Container kiểm tra Cloud CDK do MGC quản lý.

use crate::scaffold::container::error;
#[path = "clo.rs"]
mod clo;
pub(in crate::scaffold::container) use clo::write_clo_docker_assets;
