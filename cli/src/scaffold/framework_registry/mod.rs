//! Static framework directory owner.
//! Owner danh mục framework tĩnh.

mod framework_registry;

pub use framework_registry::{
    AI, AI_RUNTIMES, APP, CICD, CLOUD, GAME, HARDWARE, IOT, LIB, WEB_ALIAS, WEB_ALL, WEB_BACKEND,
    WEB_FRONTEND, WEB_FULLSTACK, ecosystem_for_core, framework_marker, iot_default_board,
    iot_framework_for_board, is_supported, supported_frameworks, template_registry_namespace,
};
