use anyhow::Result;

use super::super::super::web_command_profile_mark;
use crate::commands::core::shared;
use crate::commands::core::web::implementation::project_context::adapter_selection::web_adapter;
#[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
use crate::commands::core::web::implementation::project_context::adapter_selection::{
    web_backend_adapter_for_operation, web_backend_language,
};
use crate::commands::core::web::implementation::project_context::project_root::project_root;

/// Add web dependency
#[allow(clippy::too_many_arguments)]
pub async fn add(
    packages: Vec<String>,
    version: Option<String>,
    dev: bool,
    exact: bool,
    optional: bool,
    peer: bool,
    no_save: bool,
    install: bool,
    global: bool,
    compat_runtime: Option<String>,
) -> Result<()> {
    let started_at = std::time::Instant::now();
    let root = project_root()?;
    // Non-JS backends use the native language adapter from the Lib engine,
    // while the project remains web-owned and keeps Web security policy.
    // This avoids routing Go/Rust/Python manifests through the JS resolver.
    // (Backend ngoài JS dùng adapter ngôn ngữ của engine Lib; project vẫn
    // thuộc Web và giữ security policy Web.)
    #[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
    if let Some(backend) = web_backend_language(&root) {
        if !install {
            return Err(crate::error::web_backend_flag_unsupported(
                "--no-install",
                backend.ecosystem(),
            ));
        }
        let adapter = web_backend_adapter_for_operation(
            &root,
            backend,
            crate::commands::dep_gate::DepOp::Add,
            compat_runtime.as_deref(),
        )?;
        return shared::add(
            &*adapter, &root, packages, version, dev, exact, optional, peer, no_save, true, global,
        )
        .await;
    }
    // C0 ownership firewall (T0.3): web is native.
    // (Tường lửa C0: web native.)
    let compat = crate::commands::dep_gate::from_dep_flag(compat_runtime.as_deref())?;
    crate::commands::dep_gate::gate(
        &crate::commands::dep_gate::DepContext::new(
            "web",
            Some(crate::commands::dep_gate::eco::JS),
            None,
            None,
            crate::commands::dep_gate::DepOp::Add,
        ),
        None,
        &compat,
        Some(&root.join(".magicore").join("exec.log")),
    )?;
    let adapter = web_adapter(&root)?;
    let result = shared::add(
        &*adapter, &root, packages, version, dev, exact, optional, peer, no_save, install, global,
    )
    .await;
    web_command_profile_mark("web_add_total", started_at);
    result
}
