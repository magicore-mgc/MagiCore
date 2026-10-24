//! Static framework directory owner.
//! Owner danh mục framework tĩnh.

mod framework_registry;

pub use framework_registry::{
    AI, APP, CICD, CLOUD, GAME, HARDWARE, IOT, LIB, WEB_BACKEND, WEB_FRONTEND, ecosystem_for_core,
    framework_marker, iot_default_board, iot_framework_for_board, is_supported,
};
