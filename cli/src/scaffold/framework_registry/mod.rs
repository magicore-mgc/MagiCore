//! Static framework directory owner.
//! Owner danh mục framework tĩnh.

mod framework_registry;

pub use framework_registry::{
    detect_framework_from_markers, ecosystem_for_core, framework_marker, iot_default_board,
    iot_framework_for_board, is_known_web_unsupported, is_supported, supported_frameworks,
    template_registry_namespace, validate_all_ids_unique, validate_no_duplicates, web_all_ids,
    AI, AI_RUNTIMES, APP, CICD, CLOUD, GAME, HARDWARE, IOT, LIB, WEB_ALIAS, WEB_BACKEND,
    WEB_BACKEND_UNSUPPORTED, WEB_FRONTEND, WEB_FULLSTACK,
};

#[allow(deprecated)]
pub use framework_registry::WEB_ALL;
