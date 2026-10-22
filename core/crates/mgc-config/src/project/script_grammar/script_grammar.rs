//! Define the script grammar pin and stable ecosystem default mapping.
//! Khai báo pin grammar script và mapping mặc định ổn định theo ecosystem.

use serde::{Deserialize, Serialize};

/// Native script grammar pin — `mgc.toml [script] grammar` (Spec rev4 §4).
/// Ghim grammar script native — `mgc.toml [script] grammar` (Spec rev4 §4).
///
/// Versioned per script/core family (e.g. `web-script-v1`); adding a
/// previously-rejected construct is a compatible extension, reinterpreting a
/// previously-accepted script requires a new version + migration.
/// (Version theo họ script/core (vd `web-script-v1`); thêm cú pháp từng bị
/// từ chối là mở rộng tương thích, đổi nghĩa script đã chạy được cần version
/// mới + migration.)
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScriptGrammarConfig {
    /// Pinned grammar version (e.g. `"web-script-v1"`).
    /// Version grammar được ghim (vd `"web-script-v1"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grammar: Option<String>,
}

/// Stable default grammar per ecosystem family (Spec rev4 §4).
/// Grammar mặc định ổn định theo họ ecosystem (Spec rev4 §4).
pub fn default_script_grammar(ecosystem: &str) -> String {
    match ecosystem {
        "ai" => "ai-script-v1".to_string(),
        "app" => "app-script-v1".to_string(),
        "lib" => "lib-script-v1".to_string(),
        "game" => "game-script-v1".to_string(),
        "iot" => "iot-script-v1".to_string(),
        "cloud" | "clo" => "cloud-script-v1".to_string(),
        "cicd" => "cicd-script-v1".to_string(),
        "hardware" => "hardware-script-v1".to_string(),
        _ => "web-script-v1".to_string(),
    }
}
