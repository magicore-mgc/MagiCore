//! React Native dependency-operation gate.
//! Gate thao tác dependency cho React Native.

use anyhow::Result;
use std::path::Path;

/// Apply the shared unsupported-lane gate for React Native dependency verbs.
/// Áp dụng gate lane chưa hỗ trợ cho lệnh dependency React Native.
pub fn gate_react_native(
    root: &Path,
    lang: mgc_app_adapter::AppLanguage,
    op: crate::commands::dep_gate::DepOp,
    compat: &crate::commands::compat::CompatMode,
) -> Result<()> {
    if lang == mgc_app_adapter::AppLanguage::ReactNative {
        crate::commands::dep_gate::gate(
            &crate::commands::dep_gate::DepContext::new(
                "app",
                Some(lang.ecosystem()),
                None,
                None,
                op,
            ),
            None,
            compat,
            Some(&root.join(".magicore").join("exec.log")),
        )?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
