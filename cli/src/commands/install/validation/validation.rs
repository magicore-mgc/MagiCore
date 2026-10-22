use anyhow::Result;
use std::path::Path;

pub(in crate::commands::install) fn validate_install_request(
    adapter: &dyn mgc_types::adapter::PackageAdapter,
    project_root: &Path,
    packages: &[String],
    offline: bool,
) -> Result<()> {
    const MAX_PACKAGES: usize = 50;
    if packages.len() > MAX_PACKAGES {
        return Err(crate::error::too_many_packages(packages.len(), "install"));
    }

    // Ownership preflight must run before even acquiring the mutation
    // gateway: an unsupported offline/native lane should not create lock
    // state, recover journals, or inspect mutable manifests first.
    // (Gate trước gateway để lane unsupported không tạo side effect.)
    validate_install_owner_for_op(
        adapter,
        project_root,
        if offline {
            crate::commands::dep_gate::DepOp::OfflineReinstall
        } else {
            crate::commands::dep_gate::DepOp::Install
        },
    )?;
    reject_toolchain_owned_packages(adapter, packages)?;
    Ok(())
}

/// Cloud branch of the adapter-path firewall (T0.3-clo-gap): only CDK and
/// Pulumi projects with `package.json` may enter the native Web engine.
/// Terraform, Cloudflare, missing manifests, and unknown types fail closed.
/// (Chỉ CDK/Pulumi có `package.json` mới vào Web engine native; các nhánh
/// khác bị chặn fail-closed.)
#[cfg(feature = "clo")]
pub(in crate::commands::install) fn clo_adapter_path_gate(
    project_root: &Path,
    op: crate::commands::dep_gate::DepOp,
) -> Result<()> {
    let cloud_type = mgc_cloud_adapter::detect_type(project_root)
        .ok_or_else(|| crate::error::detect_core_failed("clo"))?;
    crate::commands::dep_gate::gate_cloud_project(project_root, cloud_type.as_str(), op, None)
}

/// No cloud support compiled in — a cloud adapter reaching this path is
/// a build-configuration error, failed closed.
/// (Không biên dịch hỗ trợ cloud — adapter cloud tới được đây là lỗi
/// cấu hình build, fail-closed.)
#[cfg(not(feature = "clo"))]
pub(in crate::commands::install) fn clo_adapter_path_gate(
    project_root: &Path,
    _op: crate::commands::dep_gate::DepOp,
) -> Result<()> {
    let _ = project_root;
    Err(crate::error::core_not_in_build("clo"))
}

/// Reject package arguments on toolchain-owned manifests (pure,
/// unit-tested): mgc cannot journal what it does not own.
/// (Từ chối packages trên manifest của tool — hàm thuần.)
pub(super) fn reject_toolchain_owned_packages(
    adapter: &dyn mgc_types::adapter::PackageAdapter,
    packages: &[String],
) -> Result<()> {
    if !packages.is_empty() && !adapter.manifest_owned() {
        return Err(crate::error::install_packages_toolchain_owned(
            adapter.name(),
        ));
    }
    Ok(())
}

#[cfg(test)]
pub(in crate::commands::install) fn validate_install_owner(
    adapter: &dyn mgc_types::adapter::PackageAdapter,
    project_root: &Path,
) -> Result<()> {
    validate_install_owner_for_op(
        adapter,
        project_root,
        crate::commands::dep_gate::DepOp::Install,
    )
}

pub(in crate::commands::install) fn validate_install_owner_for_op(
    adapter: &dyn mgc_types::adapter::PackageAdapter,
    project_root: &Path,
    op: crate::commands::dep_gate::DepOp,
) -> Result<()> {
    // C0 ownership firewall (T0.3): the adapter path serves MCP +
    // workspace/monorepo installs — it passes the same gate as the
    // per-core lanes (tool set comes from the owner table: adapter-routed
    // lanes cannot name their exact tool here).
    // Cloud branch (T0.3-clo-gap): CDK/Pulumi projects ride the native
    // web engine and skip the gate; terraform projects gate as
    // delegated; an undetected cloud type fails closed (never assumed
    // native, never silently delegated).
    // (Tường lửa C0: đường adapter phục vụ MCP + workspace — qua cùng gate
    // như lane per-core. Nhánh cloud: CDK/Pulumi đi engine web native nên
    // không qua gate; terraform gate delegate; type không nhận diện được
    // thì fail-closed.)
    {
        use mgc_types::Ecosystem;
        // Full gate context (P0#2): the adapter path serves MCP +
        // workspace/monorepo installs — ecosystem is detected per core
        // exactly like the per-core lanes (undetectable ⇒ Unsupported).
        // (Context gate đầy đủ: detect ecosystem từng core như lane
        // per-core.)
        let app_eco: Option<&str> = {
            #[cfg(feature = "app")]
            {
                mgc_app_adapter::adapter_for(project_root).map(|a| a.language.ecosystem())
            }
            #[cfg(not(feature = "app"))]
            {
                None
            }
        };
        let lib_eco: Option<&str> = {
            #[cfg(feature = "lib")]
            {
                mgc_lib_adapter::detect_language(project_root).map(|l| l.ecosystem())
            }
            #[cfg(not(feature = "lib"))]
            {
                None
            }
        };
        let iot_fw = {
            #[cfg(feature = "iot")]
            {
                mgc_iot_adapter::adapter_for(project_root).map(|a| a.framework())
            }
            #[cfg(not(feature = "iot"))]
            {
                None
            }
        };
        // P1-1: detect the REAL game engine (bevy/godot/unity/unreal)
        // instead of hardcoding Bevy — non-Bevy engines hit the owner
        // table's catch-all Unsupported branch and fail closed there,
        // never evaluated under Bevy's ownership.
        // (Detect engine game thật — không hardcode Bevy.)
        let game_eco: Option<String> = {
            #[cfg(feature = "game")]
            {
                mgc_game_adapter::adapter_for(project_root).map(|a| a.engine().to_string())
            }
            #[cfg(not(feature = "game"))]
            {
                None
            }
        };
        let (core, ecosystem, framework): (&str, Option<&str>, Option<&str>) =
            match adapter.ecosystem() {
                Ecosystem::Web => ("web", Some(crate::commands::dep_gate::eco::JS), None),
                Ecosystem::Ai => ("ai", Some(crate::commands::dep_gate::eco::PYTHON), None),
                Ecosystem::App => ("app", app_eco, None),
                Ecosystem::Lib => ("lib", lib_eco, None),
                Ecosystem::Game => {
                    let engine = game_eco.as_deref();
                    // Bevy keeps its exact historical cell; any other
                    // detected engine flows into the table's catch-all
                    // (Unsupported, fail closed) — never Bevy's.
                    // (Chỉ Bevy giữ cell cũ — engine khác fail-closed.)
                    if engine == Some(crate::commands::dep_gate::eco::BEVY) {
                        (
                            "game",
                            Some(crate::commands::dep_gate::eco::BEVY),
                            Some(crate::commands::dep_gate::eco::BEVY),
                        )
                    } else {
                        ("game", engine, engine)
                    }
                }
                // iot carries the framework id in both slots (no separate
                // language layer exists in the iot lane).
                Ecosystem::Iot => ("iot", iot_fw, iot_fw),
                Ecosystem::Cicd => ("cicd", None, None),
                Ecosystem::Hardware => ("hardware", None, None),
                Ecosystem::Cloud => ("", None, None),
            };
        let manifest_format = if core == "lib" {
            #[cfg(feature = "lib")]
            {
                mgc_lib_adapter::detect_language(project_root).and_then(|language| {
                    mgc_lib_adapter::dependency_manifest_format(project_root, language)
                })
            }
            #[cfg(not(feature = "lib"))]
            {
                None
            }
        } else if core == "ai" {
            #[cfg(feature = "ai")]
            {
                mgc_ai_adapter::uses_native_python_lane(project_root).then_some("mgc-pyproject")
            }
            #[cfg(not(feature = "ai"))]
            {
                None
            }
        } else if matches!(
            (core, ecosystem),
            ("game", Some(crate::commands::dep_gate::eco::BEVY)) | ("iot", Some("esp32-rust"))
        ) && project_root.join("Cargo.toml").is_file()
        {
            Some("cargo-toml")
        } else {
            None
        };
        if !core.is_empty() {
            let compat = crate::commands::dep_gate::from_dep_flag(None)?;
            crate::commands::dep_gate::gate(
                &crate::commands::dep_gate::DepContext::new(core, ecosystem, framework, None, op)
                    .with_manifest_format(manifest_format),
                None,
                &compat,
                Some(&project_root.join(".magicore").join("exec.log")),
            )?;
        } else {
            clo_adapter_path_gate(project_root, op)?;
        }
    }
    Ok(())
}
