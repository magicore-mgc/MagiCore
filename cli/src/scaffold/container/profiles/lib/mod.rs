//! Multi-language library lifecycle container.
//! Container lifecycle thư viện đa ngôn ngữ.

use crate::scaffold::container::error;
#[path = "lib.rs"]
mod lib;
pub(in crate::scaffold::container) use lib::write_lib_docker_assets;
