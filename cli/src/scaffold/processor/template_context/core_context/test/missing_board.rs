// Missing IoT board validation test.
// Kiểm thử validate khi thiếu board IoT.
#![allow(clippy::unwrap_used)]

use super::super::{CoreTemplateContext, ScaffoldConfig, TemplateContextError};
use std::path::PathBuf;

#[cfg(feature = "iot")]
#[test]
fn iot_context_rejects_missing_board_with_typed_error() {
    let config = ScaffoldConfig {
        core: "iot".to_string(),
        sub_type: String::new(),
        frameworks: vec!["esp32-rust".to_string()],
        project_name: "firmware".to_string(),
        features: vec![],
        template_dir: PathBuf::new(),
    };

    let error = CoreTemplateContext::try_new(&config, "firmware", "esp32-rust")
        .expect_err("missing board must not receive a fallback target");
    assert!(error.downcast_ref::<TemplateContextError>().is_some());
    assert!(error.to_string().starts_with("No board specified"));
}
