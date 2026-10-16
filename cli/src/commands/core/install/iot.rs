//! `mgc install iot` — tách từ core/iot.rs (Phase 7 v5).

use anyhow::Result;
use std::path::PathBuf;

use crate::commands::core::shared;

fn project_root() -> Result<PathBuf> {
    shared::core_project_root("iot")
}

pub async fn install(
    packages: Vec<String>,
    frozen: bool,
    compat_runtime: Option<String>,
) -> Result<()> {
    let root = project_root()?;
    let compat = crate::commands::dep_gate::from_dep_flag(compat_runtime.as_deref())?;
    // Full gate context (P0#2): the detected framework id IS the iot
    // ecosystem (no separate language layer) plus the board target.
    // Undetectable framework ⇒ Unsupported, never generic-delegated.
    // (Context gate đầy đủ: framework detect được là ecosystem iot.)
    let iot = mgc_iot_adapter::adapter_for(&root);
    let framework = iot.as_ref().map(|a| a.framework());
    let target_owned = iot.as_ref().and_then(|a| a.target(&root));
    // Native dependency lane: `mgc.lock` is canonical and `Cargo.lock` is its
    // Cargo-compatible projection. `esp32-rust` projects resolve and fetch
    // through the native crates engine with no Cargo process; `mgc build`
    // later invokes Cargo for compilation. Other frameworks remain explicitly
    // unsupported below.
    // (Lane dependency native: `mgc.lock` chuẩn, `Cargo.lock` là projection
    // tương thích Cargo. Resolve/fetch không spawn Cargo; `mgc build` dùng
    // Cargo để biên dịch.)
    if framework == Some("esp32-rust") && root.join("Cargo.toml").is_file() {
        let compat = crate::commands::dep_gate::from_dep_flag(compat_runtime.as_deref())?;
        crate::commands::dep_gate::gate(
            &crate::commands::dep_gate::DepContext::new(
                "iot",
                Some("esp32-rust"),
                Some("esp32-rust"),
                target_owned.as_deref(),
                crate::commands::dep_gate::DepOp::Install,
            )
            .with_manifest_format(Some("cargo-toml")),
            None,
            &compat,
            Some(&root.join(".magicore").join("exec.log")),
        )?;
        let lib_adapter = crate::factory::create_adapter(&mgc_types::Ecosystem::Lib, None, None)
            .map_err(|e| anyhow::anyhow!("iot native install needs the lib Cargo engine: {e}"))?;
        if !packages.is_empty() {
            crate::commands::core::shared::add(
                &*lib_adapter,
                &root,
                packages,
                None,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
            )
            .await?;
        }
        return crate::commands::core::shared::install_with_adapter(
            &*lib_adapter,
            &root,
            "mgc add",
            frozen,
            mgc_types::adapter::InstallOptions {
                legacy_flat: false,
                ..Default::default()
            },
        )
        .await;
    }
    if framework == Some("esp32-rust") {
        anyhow::bail!(
            "native ESP32-Rust dependency installation requires Cargo.toml at the project root"
        );
    }
    // Unsupported frameworks are rejected by the ownership gate before
    // mutation; `--compat-runtime` is not an implementation claim.
    crate::commands::dep_gate::gate(
        &crate::commands::dep_gate::DepContext::new(
            "iot",
            framework,
            framework,
            target_owned.as_deref(),
            crate::commands::dep_gate::DepOp::Install,
        ),
        None,
        &compat,
        Some(&root.join(".magicore").join("exec.log")),
    )?;
    let _ = packages;
    anyhow::bail!("MagiCore does not yet own dependency installation for this IoT framework")
}
