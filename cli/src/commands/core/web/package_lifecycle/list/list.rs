use anyhow::Result;

use super::super::super::web_command_profile_mark;
use crate::commands::core::shared;
use crate::commands::core::web::implementation::project_context::adapter_selection::web_adapter;
#[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
use crate::commands::core::web::implementation::project_context::adapter_selection::{
    web_backend_adapter_for_operation, web_backend_language,
};
use crate::commands::core::web::implementation::project_context::project_root::project_root;

/// List web packages
pub async fn list(compat_runtime: Option<String>) -> Result<()> {
    let started_at = std::time::Instant::now();
    let root = project_root()?;
    // Non-JS backends use the Lib language engine without changing Web ownership.
    // (Backend ngoài JS dùng engine ngôn ngữ Lib nhưng không đổi owner Web.)
    #[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
    if let Some(backend) = web_backend_language(&root) {
        let adapter = web_backend_adapter_for_operation(
            &root,
            backend,
            crate::commands::dep_gate::DepOp::List,
            compat_runtime.as_deref(),
        )?;
        return shared::list(&*adapter, &root).await;
    }
    // C0 ownership firewall (T0.3): web list is a native manifest read —
    // the explicit --compat-runtime flag is accepted for CLI uniformity
    // and ignored by the native engine (gate logs the notice).
    // (Tường lửa C0: list web đọc manifest native — cờ tường minh được
    // chấp nhận để đồng nhất CLI, engine native bỏ qua.)
    let compat = crate::commands::dep_gate::from_dep_flag(compat_runtime.as_deref())?;
    crate::commands::dep_gate::gate(
        &crate::commands::dep_gate::DepContext::new(
            "web",
            Some(crate::commands::dep_gate::eco::JS),
            None,
            None,
            crate::commands::dep_gate::DepOp::List,
        ),
        None,
        &compat,
        Some(&root.join(".magicore").join("exec.log")),
    )?;
    let adapter = web_adapter(&root)?;
    let result = shared::list(&*adapter, &root).await;
    web_command_profile_mark("web_list_total", started_at);
    result
}
