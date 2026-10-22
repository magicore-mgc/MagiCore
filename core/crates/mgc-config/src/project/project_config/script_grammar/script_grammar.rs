//! Resolve the ProjectConfig script-grammar pin.
//! Phân giải pin script grammar trong ProjectConfig.

use super::super::super::default_script_grammar;
use super::super::project_config::ProjectConfig;

impl ProjectConfig {
    /// Resolve the effective script grammar: explicit pin wins, otherwise the
    /// stable per-ecosystem default. Unknown pins are returned as-is so the
    /// caller can fail closed with the exact version string.
    /// (Xác định grammar hiệu lực: pin tường minh thắng, còn lại dùng default
    /// ổn định theo ecosystem. Pin lạ trả nguyên để caller fail-closed đúng
    /// version đó.)
    pub fn script_grammar(&self) -> String {
        if let Some(pinned) = self.script.as_ref().and_then(|s| s.grammar.clone()) {
            return pinned;
        }
        default_script_grammar(&self.ecosystem)
    }
}
