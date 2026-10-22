//! Flutter application build and test container.
//! Container build và test ứng dụng Flutter.

use crate::scaffold::container::error;
#[path = "app.rs"]
mod app;
pub(in crate::scaffold::container) use app::write_app_docker_assets;
