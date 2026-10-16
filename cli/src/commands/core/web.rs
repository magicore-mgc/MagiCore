use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result, bail};
use mgc_types::adapter::PackageAdapter;
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::commands::core::scaffold_flags::ScaffoldFlags;
use crate::commands::core::shared;
use crate::commands::freshness::check_profile_gate;
use mgc_types::Ecosystem;
use mgc_ui::info;

// P1 split (2026-09-11): npm version resolution lives in web/versions.rs.
mod versions;

fn web_command_profile_enabled() -> bool {
    std::env::var_os("MAGICORE_WEB_PROFILE_COMMAND").is_some()
}

fn web_command_profile_mark(label: &str, started_at: std::time::Instant) {
    if web_command_profile_enabled() {
        eprintln!(
            "[magicore:web:web-command-profile] {}={}ms",
            label,
            started_at.elapsed().as_millis()
        );
    }
}

fn install_hint_command() -> &'static str {
    #[cfg(all(
        feature = "web",
        not(any(
            feature = "game",
            feature = "ai",
            feature = "clo",
            feature = "cicd",
            feature = "iot",
            feature = "app",
            feature = "lib"
        ))
    ))]
    {
        return "mgc install";
    }

    #[cfg(all(
        feature = "web",
        any(
            feature = "game",
            feature = "ai",
            feature = "clo",
            feature = "cicd",
            feature = "iot",
            feature = "app",
            feature = "lib"
        )
    ))]
    {
        return "mgc install-web";
    }

    #[allow(unreachable_code)]
    "mgc install"
}

/// Find project root for web commands
fn project_root() -> Result<std::path::PathBuf> {
    let started_at = std::time::Instant::now();
    let cwd = std::env::current_dir().map_err(|e| crate::error::cwd_deleted(&e))?;
    let root = shared::find_project_root(&cwd)?
        .ok_or_else(|| crate::error::no_mgc_project_found("web"))?;
    shared::ensure_project_core_identity(&root, "web")?;
    web_command_profile_mark("project_root", started_at);
    Ok(root)
}

/// Non-JS backend language of a web project, when `package.json` is
/// absent (create-web backend scaffolds: gin/echo/fiber → Go,
/// axum/actix → Rust, fastapi/django/flask → Python, spring/quarkus →
/// Java, dotnet-* → .NET). `None` = JS/TS lane owns it (or nothing
/// detectable — current behavior runs and fails honestly downstream).
/// Detected from manifest markers first, then mgc.toml frameworks[].
/// (Ngôn ngữ backend non-JS của project web khi không có package.json.)
#[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
fn web_backend_language(root: &Path) -> Option<mgc_lib_adapter::LibLanguage> {
    use mgc_lib_adapter::LibLanguage;
    if root.join("package.json").exists() {
        return None;
    }
    if root.join("go.mod").exists() {
        return Some(LibLanguage::Go);
    }
    if root.join("Cargo.toml").exists() {
        return Some(LibLanguage::Rust);
    }
    if root.join("pyproject.toml").exists()
        || root.join("setup.py").exists()
        || root.join("requirements.txt").exists()
    {
        return Some(LibLanguage::Python);
    }
    if root.join("pom.xml").exists()
        || root.join("build.gradle").exists()
        || root.join("build.gradle.kts").exists()
    {
        return Some(LibLanguage::Java);
    }
    if std::fs::read_dir(root)
        .into_iter()
        .flatten()
        .flatten()
        .any(|e| e.path().extension().is_some_and(|ext| ext == "csproj"))
    {
        return Some(LibLanguage::DotNet);
    }
    let mgc_toml =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
            .ok()??;
    let value: toml::Value = toml::from_str(&mgc_toml).ok()?;
    let frameworks: Vec<&str> = value
        .get("frameworks")
        .and_then(|f| f.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default();
    frameworks.into_iter().find_map(|framework| {
        let base = framework.split('/').next_back().unwrap_or(framework);
        match base {
            "gin" | "echo" | "fiber" | "go" => Some(LibLanguage::Go),
            "axum" | "actix-web" | "actix" | "rust" => Some(LibLanguage::Rust),
            "fastapi" | "django" | "flask" | "python" => Some(LibLanguage::Python),
            "spring-boot" | "spring" | "quarkus" | "java" => Some(LibLanguage::Java),
            "dotnet-webapi" | "dotnet-minimal" | "dotnet" | "csharp" => Some(LibLanguage::DotNet),
            _ => None,
        }
    })
}

/// Build the native language adapter for a web-owned backend project.
/// Dependency ownership is evaluated by the lib language lane, while the
/// project identity and adapter security policy remain `web`.
/// Dựng adapter ngôn ngữ native cho backend do web sở hữu. Gate dùng lane
/// ngôn ngữ lib; project identity và security policy vẫn thuộc `web`.
#[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
fn web_backend_adapter_for_operation(
    root: &Path,
    language: mgc_lib_adapter::LibLanguage,
    operation: crate::commands::dep_gate::DepOp,
    compat_runtime: Option<&str>,
) -> Result<Arc<dyn PackageAdapter>> {
    let compat = crate::commands::dep_gate::from_dep_flag(compat_runtime)?;
    // Gate the same language and manifest that web_backend_language selected
    // for the adapter; Lib's generic detector has a different probe order.
    // (Gate theo đúng ngôn ngữ/manifest Web đã chọn; detector Lib tổng quát
    // có thứ tự dò khác.)
    let context = crate::commands::dep_gate::DepContext::new(
        "lib",
        Some(language.ecosystem()),
        None,
        None,
        operation,
    )
    .with_manifest_format(mgc_lib_adapter::dependency_manifest_format(root, language));
    crate::commands::dep_gate::gate(
        &context,
        None,
        &compat,
        Some(&root.join(".magicore").join("exec.log")),
    )?;
    let adapter = mgc_lib_adapter::adapter_for_web_backend(language, root)?;
    Ok(Arc::new(adapter))
}

fn web_adapter(root: &Path) -> Result<Arc<dyn PackageAdapter>> {
    let started_at = std::time::Instant::now();
    let config = mgc_config::project::ProjectConfig::load(root)?;
    let chain = mgc_config::chain::registry_chain_scoped_checked(Some(root), config.as_ref())?;
    let adapter = crate::factory::create_adapter_for_scoped(root, &Ecosystem::Web, chain)?;
    web_command_profile_mark("web_adapter", started_at);
    Ok(adapter)
}

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

/// Remove web dependencies
pub async fn remove(
    packages: Vec<String>,
    install: bool,
    compat_runtime: Option<String>,
) -> Result<()> {
    let started_at = std::time::Instant::now();
    let root = project_root()?;
    // Non-JS backends use the Lib language engine without changing Web ownership.
    // (Backend ngoài JS dùng engine ngôn ngữ Lib nhưng không đổi owner Web.)
    #[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
    if let Some(backend) = web_backend_language(&root) {
        let adapter = web_backend_adapter_for_operation(
            &root,
            backend,
            crate::commands::dep_gate::DepOp::Remove,
            compat_runtime.as_deref(),
        )?;
        return shared::remove(&*adapter, &root, packages, install).await;
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
            crate::commands::dep_gate::DepOp::Remove,
        ),
        None,
        &compat,
        Some(&root.join(".magicore").join("exec.log")),
    )?;
    let adapter = web_adapter(&root)?;
    let result = shared::remove(&*adapter, &root, packages, install).await;
    web_command_profile_mark("web_remove_total", started_at);
    result
}

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

/// Update web packages
pub async fn update(
    packages: Vec<String>,
    install: bool,
    compat_runtime: Option<String>,
) -> Result<()> {
    let started_at = std::time::Instant::now();
    let root = project_root()?;
    // Non-JS backend: the lib lane owns this lifecycle (see add()).
    // (Backend non-JS: giao cho lane lib.)
    #[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
    if let Some(backend) = web_backend_language(&root) {
        let adapter = web_backend_adapter_for_operation(
            &root,
            backend,
            crate::commands::dep_gate::DepOp::Update,
            compat_runtime.as_deref(),
        )?;
        return shared::update(&*adapter, &root, packages, install).await;
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
            crate::commands::dep_gate::DepOp::Update,
        ),
        None,
        &compat,
        Some(&root.join(".magicore").join("exec.log")),
    )?;
    let adapter = web_adapter(&root)?;
    let result = shared::update(&*adapter, &root, packages, install).await;
    web_command_profile_mark("web_update_total", started_at);
    result
}

#[allow(clippy::too_many_arguments)]
pub async fn install(
    packages: Vec<String>,
    frozen: bool,
    ignore_scripts: bool,
    allow_scripts: bool,
    prefer_dedupe: bool,
    repair: bool,
    offline: bool,
    compat_runtime: Option<String>,
) -> Result<()> {
    // C0 ownership firewall (T0.3): web is native — the gate records the
    // decision and warns if compat was passed pointlessly.
    // (Tường lửa C0: web native — gate ghi nhận và cảnh báo nếu compat
    // thừa.)
    let root = project_root()?;
    if offline {
        crate::offline::set_offline_mode(true);
        if !packages.is_empty() {
            anyhow::bail!("offline install cannot add packages; run `mgc add` online first");
        }
    }
    // Non-JS backends use the Lib language engine without changing Web ownership.
    // Web-pipeline-only flags have no meaning there — fail loudly.
    // (Backend ngoài JS dùng engine Lib nhưng vẫn thuộc Web; flag riêng JS fail rõ.)
    #[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
    if let Some(backend) = web_backend_language(&root) {
        let language = backend.ecosystem();
        if ignore_scripts {
            return Err(crate::error::web_backend_flag_unsupported(
                "--ignore-scripts",
                language,
            ));
        }
        if allow_scripts {
            return Err(crate::error::web_backend_flag_unsupported(
                "--allow-scripts",
                language,
            ));
        }
        if prefer_dedupe {
            return Err(crate::error::web_backend_flag_unsupported(
                "--prefer-dedupe",
                language,
            ));
        }
        if repair {
            return Err(crate::error::web_backend_flag_unsupported(
                "--repair", language,
            ));
        }
        if !packages.is_empty() {
            return Err(crate::error::install_lib_packages_use_add(&packages));
        }
        let operation = if offline {
            crate::commands::dep_gate::DepOp::OfflineReinstall
        } else {
            crate::commands::dep_gate::DepOp::Install
        };
        let adapter = web_backend_adapter_for_operation(
            &root,
            backend,
            operation,
            compat_runtime.as_deref(),
        )?;
        return shared::install_with_adapter(
            &*adapter,
            &root,
            "mgc add",
            frozen || offline,
            mgc_types::adapter::InstallOptions {
                legacy_flat: false,
                offline,
                ..Default::default()
            },
        )
        .await;
    }
    let compat = crate::commands::dep_gate::from_dep_flag(compat_runtime.as_deref())?;
    crate::commands::dep_gate::gate(
        &crate::commands::dep_gate::DepContext::new(
            "web",
            Some(crate::commands::dep_gate::eco::JS),
            None,
            None,
            crate::commands::dep_gate::DepOp::Install,
        ),
        None,
        &compat,
        Some(&root.join(".magicore").join("exec.log")),
    )?;
    let adapter: Arc<dyn PackageAdapter> = web_adapter(&root)?;
    let targets = install_targets(&root)?;

    // Reject unsupported mixed-stack members before the frontend manifest,
    // lockfile, or shared store can be changed.
    // (Từ chối member chưa hỗ trợ trước khi sửa manifest, lock hay store của frontend.)
    for target in &targets {
        if !target.join("package.json").exists() {
            compat_install_target(target, &compat)?;
        }
    }

    // Compat gate for monorepo native (non-package.json) members: the
    // `compat` resolved for the dep_gate above travels down (not a
    // bool) so each toolchain branch opens ONLY for its exact tool —
    // one flag never opens every branch.
    // (Dùng lại compat của gate — mỗi nhánh chỉ mở đúng tool của nó.)

    // Dedupe opt-in (02 §2.1): CLI flag OR mgc.toml [dedupe] prefer = true.
    let mut dedupe_enabled = prefer_dedupe;
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if !dedupe_enabled && let Ok(Some(cfg)) = mgc_config::project::ProjectConfig::load(&root) {
        dedupe_enabled = cfg.dedupe.prefer;
    }
    if dedupe_enabled {
        adapter.set_dedupe_pref(true);
        // Shared lock seeding: root mgc.lock (workspace aggregate) + mỗi web target lock.
        let mut lock_roots: Vec<&std::path::Path> = vec![root.as_path()];
        lock_roots.extend(
            targets
                .iter()
                .filter(|target| target.join("package.json").exists())
                .map(|target| target.as_path()),
        );
        // Issue #4: existing_versions_from disabled - restore after lockfile v2 migration complete
        // if let Ok(existing) = mgc_lockfile::existing_versions_from(&lock_roots) {
        if let Ok(_existing) = Ok::<Vec<String>, ()>(Vec::new()) {
            // Skipping set_existing_versions call until v2 migration complete
            // if !existing.is_empty() {
            //     adapter.set_existing_versions(existing);
            // }
        }
    }
    // Root-manifest package adds go through the shared mutation
    // gateway (P0-1): adapter.add() here would parse → edit → write the
    // manifest with no lock, no recovery, no journal — the exact generic
    // bypass, racing concurrent mutations. shared::add resolves +
    // rewrites identically, under lock + journal; install=false leaves
    // the install tail to this lane's own flow below.
    // (Add root manifest qua gateway chung — không adapter.add trực tiếp.)
    if !packages.is_empty() {
        shared::add(
            adapter.as_ref(),
            &root,
            packages.clone(),
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

    let project_mode = detect_project_mode(&root)?;
    if matches!(project_mode, WebProjectMode::Monorepo) {
        // Mix core (Q23): mỗi workspace project detect core riêng — web targets
        // đi qua web adapter, target khác (lib/app/ai/...) qua adapter của nó.
        let mut web_targets = Vec::new();
        let mut mix_targets = Vec::new();
        for target in &targets {
            if target.join("package.json").exists() {
                web_targets.push(target.clone());
            } else {
                mix_targets.push(target.clone());
            }
        }

        if offline && !mix_targets.is_empty() {
            anyhow::bail!(
                "offline install for mixed-core monorepos is not yet supported; no workspace install was started"
            );
        }

        if !web_targets.is_empty() {
            install_monorepo_targets(MonorepoInstallParams {
                adapter: Arc::clone(&adapter),
                targets: web_targets.clone(),
                frozen,
                offline,
                ignore_scripts,
                allow_scripts,
                prefer_dedupe,
                repair,
                compat: compat.clone(),
            })
            .await?;
            link_monorepo_workspace_packages(&root, &web_targets)?;
            write_monorepo_root_lockfile(&root, &web_targets)?;
        }

        for target in &mix_targets {
            let ctx = crate::context::ProjectContext::load_for_dir(target)?;
            mgc_ui::info(&format!("Installing workspace: {}", target.display()));
            crate::commands::install::install_into_root_ws(
                ctx.adapter(),
                target,
                &packages,
                ignore_scripts,
                allow_scripts,
                offline,
                frozen, // frozen mode
            )
            .await?;
        }
    } else {
        for target in &targets {
            if target.join("package.json").exists() {
                shared::install_with_adapter(
                    adapter.as_ref(),
                    target,
                    "mgc add",
                    frozen || offline,
                    mgc_types::adapter::InstallOptions {
                        allow_scripts,
                        prefer_dedupe: dedupe_enabled,
                        repair,
                        legacy_flat: shared::should_use_legacy_flat_layout("web"),
                        offline,
                        ..Default::default()
                    },
                )
                .await?;
            } else {
                compat_install_target(target, &compat)?;
            }
        }
    }

    Ok(())
}

/// Pre-flight freshness: every dependency declared in `package.json`
/// must have a locked entry in `mgc.lock` (Spec rev4 §8).
/// Only dependency metadata participates — editing `scripts.*` alone never
/// triggers a resolve. Missing/invalid lock fails closed.
/// (Freshness pre-flight: mọi dependency khai báo trong `package.json` phải
/// có entry trong `mgc.lock` (Spec rev4 §8). Chỉ metadata dependency được so —
/// sửa `scripts.*` không trigger resolve. Thiếu/sai lock thì fail-closed.)
pub fn check_dev_freshness(project_root: &Path) -> Result<()> {
    let manifest_path = project_root.join("package.json");
    if !manifest_path.exists() {
        return Ok(());
    }
    let manifest: Value = serde_json::from_str(
        &std::fs::read_to_string(&manifest_path)
            .with_context(|| format!("cannot read {}", manifest_path.display()))?,
    )
    .with_context(|| format!("{} is not valid JSON", manifest_path.display()))?;

    let declared = crate::commands::freshness::declared_npm_dep_specs(&manifest)?;
    crate::commands::freshness::check_dev_npm_freshness_against_lock(project_root, &declared)
}

pub async fn dev_at_root(
    project_root: &Path,
    host: Option<String>,
    port: Option<u16>,
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
) -> Result<()> {
    // Refresh policy distribution once, resolve every launch target, then
    // finish freshness and profile pre-flight for each target before any
    // lifecycle hook or dev process can spawn.
    // (Refresh policy một lần, resolve toàn bộ target, rồi hoàn tất freshness
    // và profile pre-flight từng target trước khi spawn hook hay dev process.)
    crate::commands::policy::refresh_url_manifest_if_configured(project_root).await;
    let targets = dev_targets(project_root, host, port)?;
    let profile_store = crate::commands::freshness::default_profile_store_dir();
    for target in &targets {
        check_dev_freshness(&target.dir)?;
        // Pre-flight row 3: known-mutation lanes refuse upfront (never mid-run).
        // (Hàng 3: lane mutation đã biết từ chối upfront, không kill giữa run.)
        check_profile_gate(&target.dir, &profile_store, "dev", compat_runner)?;
    }
    if targets.len() == 1 {
        return run_single_dev_target(&targets[0], compat, compat_runner).await;
    }

    run_multi_dev_targets(&targets, compat, compat_runner).await
}

#[derive(Debug)]
struct DevLaunch {
    program: PathBuf,
    args: Vec<OsString>,
    envs: Vec<(OsString, OsString)>,
}

impl DevLaunch {
    fn describe(&self) -> String {
        let args = self
            .args
            .iter()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");
        if args.is_empty() {
            self.program.display().to_string()
        } else {
            format!("{} {}", self.program.display(), args)
        }
    }
}

#[derive(Debug, Deserialize)]
struct DevPackageJson {
    scripts: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct WorkspaceConfig {
    mode: Option<String>,
    layout: Option<WorkspaceLayout>,
    workspace: Option<WorkspaceManifest>,
}

#[derive(Debug, Deserialize)]
struct WorkspaceLayout {
    apps_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WorkspaceManifest {
    apps: Option<Vec<String>>,
    packages: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
struct DevTarget {
    dir: PathBuf,
    role: &'static str,
    script_name: &'static str,
    host: Option<String>,
    port: Option<u16>,
}

#[derive(Debug, Clone)]
enum WebProjectMode {
    Standalone,
    FullstackSplit,
    Monorepo,
}

fn detect_dev_target(project_root: &Path) -> Result<PathBuf> {
    let root_script = read_dev_script(project_root)?;
    if root_script
        .as_deref()
        .is_some_and(|script| !script.starts_with("mgc "))
    {
        return Ok(project_root.to_path_buf());
    }

    let frontend = workspace_frontend_dir(project_root)?;
    if frontend.join("package.json").exists() {
        return Ok(frontend);
    }

    if root_script.is_some() {
        return Ok(project_root.to_path_buf());
    }

    if infer_native_dev_launch(project_root, None, None).is_ok() {
        return Ok(project_root.to_path_buf());
    }

    Err(crate::error::web_no_dev_target(
        project_root,
        install_hint_command(),
    ))
}

fn workspace_frontend_dir(project_root: &Path) -> Result<PathBuf> {
    if let Some(contents) = mgc_workspace::read_workspace_config(project_root)? {
        let config: WorkspaceConfig = toml::from_str(&contents)?;
        if is_workspace_monorepo(&config) {
            let apps_dir = config
                .layout
                .as_ref()
                .and_then(|layout| layout.apps_dir.as_deref())
                .unwrap_or("apps");
            let apps_dir =
                mgc_workspace::resolve_workspace_layout_dir(project_root, apps_dir, "apps_dir")?;
            let frontend = apps_dir.join("frontend");
            mgc_workspace::workspace_entry_is_directory(&frontend)?;
            return Ok(frontend);
        }
    }
    Ok(project_root.join("apps").join("frontend"))
}

fn workspace_backend_dir(project_root: &Path) -> Result<PathBuf> {
    if let Some(contents) = mgc_workspace::read_workspace_config(project_root)? {
        let config: WorkspaceConfig = toml::from_str(&contents)?;
        if is_workspace_monorepo(&config) {
            let apps_dir = config
                .layout
                .as_ref()
                .and_then(|layout| layout.apps_dir.as_deref())
                .unwrap_or("apps");
            let apps_dir =
                mgc_workspace::resolve_workspace_layout_dir(project_root, apps_dir, "apps_dir")?;
            let backend = apps_dir.join("backend");
            mgc_workspace::workspace_entry_is_directory(&backend)?;
            return Ok(backend);
        }
    }
    Ok(project_root.join("apps").join("backend"))
}

fn detect_project_mode(project_root: &Path) -> Result<WebProjectMode> {
    if let Some(contents) = mgc_workspace::read_workspace_config(project_root)? {
        let config: WorkspaceConfig = toml::from_str(&contents)?;
        if is_workspace_monorepo(&config) {
            return Ok(WebProjectMode::Monorepo);
        }
    }

    if has_backend_manifest(&project_root.join("server")) {
        return Ok(WebProjectMode::FullstackSplit);
    }

    Ok(WebProjectMode::Standalone)
}

fn install_targets(project_root: &Path) -> Result<Vec<PathBuf>> {
    match detect_project_mode(project_root)? {
        WebProjectMode::Standalone => Ok(vec![project_root.to_path_buf()]),
        WebProjectMode::FullstackSplit => Ok(vec![
            project_root.to_path_buf(),
            project_root.join("server"),
        ]),
        WebProjectMode::Monorepo => discover_monorepo_install_targets(project_root),
    }
}

fn is_workspace_monorepo(config: &WorkspaceConfig) -> bool {
    if config.mode.as_deref() == Some("monorepo") {
        return true;
    }

    config.workspace.as_ref().is_some_and(|workspace| {
        workspace.apps.as_ref().is_some_and(|apps| !apps.is_empty())
            || workspace
                .packages
                .as_ref()
                .is_some_and(|packages| !packages.is_empty())
    })
}

fn discover_monorepo_install_targets(project_root: &Path) -> Result<Vec<PathBuf>> {
    mgc_workspace::discover_workspace_targets(project_root)
}

#[derive(Debug, Deserialize)]
struct WorkspacePackageJson {
    name: String,
    #[serde(default)]
    dependencies: std::collections::HashMap<String, String>,
    #[serde(default, rename = "devDependencies")]
    dev_dependencies: std::collections::HashMap<String, String>,
    #[serde(default, rename = "peerDependencies")]
    peer_dependencies: std::collections::HashMap<String, String>,
    #[serde(default, rename = "optionalDependencies")]
    optional_dependencies: std::collections::HashMap<String, String>,
}

fn link_monorepo_workspace_packages(project_root: &Path, targets: &[PathBuf]) -> Result<()> {
    let package_targets = targets
        .iter()
        .filter(|path| path.join("package.json").exists())
        .cloned()
        .collect::<Vec<_>>();
    if package_targets.is_empty() {
        return Ok(());
    }

    let mut workspace_index = std::collections::HashMap::<String, PathBuf>::new();
    let mut manifests = Vec::new();

    for target in &package_targets {
        let manifest = read_workspace_package_manifest(target)?;
        workspace_index.insert(manifest.name.clone(), target.clone());
        manifests.push((target.clone(), manifest));
    }

    for (target, manifest) in manifests {
        let node_modules = target.join("node_modules");
        std::fs::create_dir_all(&node_modules)?;

        for (dep_name, spec) in manifest
            .dependencies
            .into_iter()
            .chain(manifest.dev_dependencies)
            .chain(manifest.peer_dependencies)
            .chain(manifest.optional_dependencies)
        {
            if !spec.trim().starts_with("workspace:") {
                continue;
            }
            let Some(source_dir) = workspace_index.get(&dep_name) else {
                return Err(crate::error::web_workspace_dep_missing(
                    &dep_name,
                    &target,
                    project_root,
                ));
            };

            let link_path = node_modules.join(dep_name.replace('/', std::path::MAIN_SEPARATOR_STR));
            if let Some(parent) = link_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            if link_path.exists() {
                let metadata = std::fs::symlink_metadata(&link_path)?;
                if metadata.file_type().is_symlink() || metadata.is_file() {
                    std::fs::remove_file(&link_path)?;
                } else if metadata.is_dir() {
                    std::fs::remove_dir_all(&link_path)?;
                }
            }
            create_workspace_dir_link(source_dir, &link_path)?;
        }
    }

    Ok(())
}

fn read_workspace_package_manifest(project_root: &Path) -> Result<WorkspacePackageJson> {
    let path = project_root.join("package.json");
    let contents = std::fs::read_to_string(&path).with_context(|| {
        format!(
            "failed to read workspace package manifest '{}'",
            path.display()
        )
    })?;
    serde_json::from_str(&contents).with_context(|| {
        format!(
            "failed to parse workspace package manifest '{}'",
            path.display()
        )
    })
}

struct MonorepoInstallParams {
    adapter: Arc<dyn PackageAdapter>,
    targets: Vec<PathBuf>,
    frozen: bool,
    offline: bool,
    ignore_scripts: bool,
    allow_scripts: bool,
    prefer_dedupe: bool,
    repair: bool,
    compat: crate::commands::compat::CompatMode,
}

async fn install_monorepo_targets(params: MonorepoInstallParams) -> Result<()> {
    let mut native_targets = Vec::new();
    let mut package_targets = Vec::new();

    for target in &params.targets {
        if target.join("package.json").exists() {
            package_targets.push(target.clone());
        } else {
            native_targets.push(target.clone());
        }
    }

    if params.offline && !native_targets.is_empty() {
        anyhow::bail!(
            "offline install cannot use delegated non-package workspace members; no workspace install was started"
        );
    }

    if !package_targets.is_empty() {
        let graph = mgc_workspace::build_workspace_graph(&package_targets)?;
        let levels =
            mgc_workspace::topo_levels(&graph).map_err(|e| crate::error::topo_order_failed(&e))?;

        let concurrency = monorepo_install_concurrency(&package_targets);
        let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));

        for level in &levels {
            let mut join_set = tokio::task::JoinSet::new();
            for &node_index in level {
                let node = graph.nodes[node_index].clone();
                let adapter = Arc::clone(&params.adapter);
                let semaphore = Arc::clone(&semaphore);
                join_set.spawn(async move {
                    let _permit = semaphore
                        .acquire_owned()
                        .await
                        .map_err(|e| crate::error::install_slot_failed(&e))?;
                    mgc_ui::info(&format!("Installing workspace: {}", node.path.display()));
                    let install_options = mgc_types::adapter::InstallOptions {
                        ignore_scripts: params.ignore_scripts,
                        allow_scripts: params.allow_scripts,
                        prefer_dedupe: params.prefer_dedupe,
                        repair: params.repair,
                        legacy_flat: shared::should_use_legacy_flat_layout("web"),
                        frozen: params.frozen,
                        offline: params.offline,
                        ..Default::default()
                    };
                    install_web_target_quiet(
                        adapter.as_ref(),
                        &node.path,
                        params.frozen,
                        install_options,
                    )
                    .await?;
                    Ok::<PathBuf, anyhow::Error>(node.path)
                });
            }
            while let Some(result) = join_set.join_next().await {
                result.map_err(|e| crate::error::install_task_failed(&e))??;
            }
        }
    }

    for target in native_targets {
        compat_install_target(&target, &params.compat)?;
    }

    Ok(())
}

fn monorepo_install_concurrency(package_targets: &[PathBuf]) -> usize {
    if package_targets.is_empty() {
        return 1;
    }

    if let Some(override_value) = std::env::var("MAGICORE_WEB_MONOREPO_INSTALL_CONCURRENCY")
        .ok()
        .and_then(|raw| raw.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
    {
        return override_value;
    }

    let default = std::thread::available_parallelism()
        .map(|count| count.get().min(4))
        .unwrap_or(2)
        .max(1);

    if looks_like_cold_monorepo_install(package_targets) {
        return 1;
    }

    default
}

fn looks_like_cold_monorepo_install(package_targets: &[PathBuf]) -> bool {
    if package_targets.len() <= 1 {
        return false;
    }

    package_targets.iter().all(|target| {
        !target.join("node_modules").exists()
            && !target.join("mgc.lock").exists()
            && !target.join(".magicore").join("cache").join("web").exists()
    })
}

async fn install_web_target_quiet(
    adapter: &dyn PackageAdapter,
    target: &Path,
    frozen: bool,
    opts: mgc_types::adapter::InstallOptions,
) -> Result<()> {
    // Each workspace target is its own project: acquire its mutation gateway
    // before resolution and the empty-graph return, so recovery and concurrent
    // installs cannot bypass that target's writer lock.
    // (Mỗi target là một project riêng: vào mutation gateway trước resolve
    // và nhánh graph rỗng để recovery/install đồng thời không bypass writer lock.)
    let _write_lock =
        shared::begin_dependency_mutation(adapter, target, shared::MutationOperation::Install)
            .await?;
    let execution =
        shared::prepare_install_execution(adapter, target, frozen || opts.offline, None).await?;
    if execution.graph.is_empty() {
        return Ok(());
    }
    adapter.install(&execution.graph, target, opts).await?;
    Ok(())
}

// Issue #4: Disabled due to lockfile v2 migration (uses LockPackage, WorkspaceLock, ResolutionMeta)
fn write_monorepo_root_lockfile(_project_root: &Path, _targets: &[PathBuf]) -> Result<()> {
    // Workspace lockfile merging requires v2 schema rewrite
    // For now, each workspace maintains its own lockfile
    Ok(())
}

#[cfg(unix)]
fn create_workspace_dir_link(source: &Path, target: &Path) -> Result<()> {
    std::os::unix::fs::symlink(source, target).with_context(|| {
        format!(
            "failed to symlink workspace package '{}' -> '{}'",
            target.display(),
            source.display()
        )
    })?;
    Ok(())
}

#[cfg(windows)]
fn create_workspace_dir_link(source: &Path, target: &Path) -> Result<()> {
    std::os::windows::fs::symlink_dir(source, target).with_context(|| {
        format!(
            "failed to symlink workspace package '{}' -> '{}'",
            target.display(),
            source.display()
        )
    })?;
    Ok(())
}

fn dev_targets(
    project_root: &Path,
    host: Option<String>,
    port: Option<u16>,
) -> Result<Vec<DevTarget>> {
    // Backend port từ bảng trung tâm dev_port (RULE §13/§12 — không hardcode).
    // Backend port comes from the centralized dev_port table.
    let fullstack_backend_port = Some(crate::commands::core::dev_port::PORT_WEB_BE);
    let monorepo_backend_port = Some(crate::commands::core::dev_port::PORT_WEB_BE);

    match detect_project_mode(project_root)? {
        WebProjectMode::Standalone => Ok(vec![DevTarget {
            dir: detect_dev_target(project_root)?,
            role: "frontend",
            script_name: "dev",
            host,
            port,
        }]),
        WebProjectMode::FullstackSplit => Ok(vec![
            DevTarget {
                dir: project_root.to_path_buf(),
                role: "frontend",
                script_name: "dev:client",
                host,
                port,
            },
            DevTarget {
                dir: project_root.join("server"),
                role: "backend",
                script_name: "dev",
                host: None,
                port: fullstack_backend_port,
            },
        ]),
        WebProjectMode::Monorepo => Ok(vec![
            DevTarget {
                dir: workspace_frontend_dir(project_root)?,
                role: "frontend",
                script_name: "dev",
                host,
                port,
            },
            DevTarget {
                dir: workspace_backend_dir(project_root)?,
                role: "backend",
                script_name: "dev",
                host: None,
                port: monorepo_backend_port,
            },
        ]),
    }
}

fn read_script(project_root: &Path, script_name: &str) -> Result<Option<String>> {
    let package_json_path = project_root.join("package.json");
    if !package_json_path.exists() {
        return Ok(None);
    }

    let parsed: DevPackageJson =
        serde_json::from_str(&std::fs::read_to_string(package_json_path)?)?;
    let scripts = match parsed.scripts {
        Some(scripts) => scripts,
        None => return Ok(None),
    };

    if let Some(script) = scripts.get(script_name).cloned() {
        return Ok(Some(script));
    }

    if script_name == "dev" {
        return Ok(scripts.get("start").cloned());
    }

    Ok(None)
}

fn read_dev_script(project_root: &Path) -> Result<Option<String>> {
    read_script(project_root, "dev")
}

fn replace_dev_endpoint_arg(args: &mut Vec<OsString>, flag: &str, value: OsString) {
    let equals_prefix = format!("{flag}=");
    let mut normalized = Vec::with_capacity(args.len() + 2);
    let mut index = 0;

    while index < args.len() {
        let current = args[index].to_string_lossy();
        if current == flag {
            index += 1;
            if index < args.len() && !args[index].to_string_lossy().starts_with('-') {
                index += 1;
            }
        } else if current.starts_with(&equals_prefix) {
            index += 1;
        } else {
            normalized.push(args[index].clone());
            index += 1;
        }
    }

    normalized.push(OsString::from(flag));
    normalized.push(value);
    *args = normalized;
}

fn append_dev_endpoint_args(
    args: &mut Vec<OsString>,
    host_flag: &str,
    port_flag: &str,
    host: Option<String>,
    port: Option<u16>,
) {
    // MGC owns the selected endpoint, so replace script values in both `--flag value` and `--flag=value` forms.
    // MGC sở hữu endpoint đã chọn nên thay giá trị trong cả hai dạng cờ của script.
    if let Some(host) = host {
        replace_dev_endpoint_arg(args, host_flag, OsString::from(host));
    }
    if let Some(port) = port {
        replace_dev_endpoint_arg(args, port_flag, OsString::from(port.to_string()));
    }
}

/// Validate runtime args for Bun/Deno using shared launcher policy
/// Kiểm tra args runtime cho Bun/Deno dùng policy launcher chung
///
/// SAFETY: Uses centralized launcher_policy module for consistent validation
/// AN TOÀN: Dùng module launcher_policy tập trung để kiểm tra nhất quán
fn validate_runtime_args(runtime: &str, args: &[&str]) -> Result<()> {
    use crate::commands::launcher_policy::{LauncherPolicy, Runtime};

    let rt = match runtime {
        "bun" => Runtime::Bun,
        "deno" => Runtime::Deno,
        "node" => Runtime::Node,
        _ => return Ok(()), // Unknown runtime, skip validation
    };

    let policy = LauncherPolicy::dev_server(rt);
    policy.validate_args(args)
}

/// Detect runtime from script tokens for optimizer env loading
/// Phát hiện runtime từ script tokens để load env optimizer
fn detect_runtime_from_tokens(
    tokens: &[&str],
) -> crate::commands::optimizer::runtime_detect::DetectedRuntime {
    use crate::commands::optimizer::runtime_detect::{DetectedRuntime, PackageManager};

    if tokens.is_empty() {
        return DetectedRuntime::Unknown;
    }

    match tokens[0] {
        "bun" => DetectedRuntime::Bun,
        "deno" => DetectedRuntime::Deno,
        "node" | "npm" | "pnpm" | "yarn" | "vite" | "next" | "webpack" | "react-scripts" => {
            DetectedRuntime::NodeJs {
                package_manager: PackageManager::Npm, // Default, actual PM doesn't matter for env loading
            }
        }
        _ => DetectedRuntime::Unknown,
    }
}

fn build_dev_launch(
    project_root: &Path,
    script_name: &str,
    host: Option<String>,
    port: Option<u16>,
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
) -> Result<DevLaunch> {
    let script = match read_script(project_root, script_name)? {
        Some(script) => script,
        None => return infer_native_dev_launch(project_root, host, port),
    };

    reject_external_package_manager_script(
        &script,
        &project_root.join("package.json"),
        compat_runner,
    )?;

    let tokens: Vec<&str> = script.split_whitespace().collect();
    if tokens.is_empty() {
        return Err(crate::error::web_empty_dev_script(project_root));
    }

    // NATIVE-ENGINE GATE (P0-1 fix 2026-09-10): nếu PROGRAM của script là
    // runtime đối thủ (bun/deno) thì phải qua cổng compat TRƯỚC khi dựng
    // DevLaunch — native mode fail trước process spawn, compat mode chỉ mở
    // đúng runtime đã chọn + cảnh báo lớn.
    if let Some(program) = tokens.first() {
        crate::commands::compat::gate_runtime_spawn(compat, program)?;
    }

    // Detect runtime from script to load correct optimizer config
    // Phát hiện runtime từ script để load đúng config optimizer
    let runtime = detect_runtime_from_tokens(&tokens);
    let optimizer_envs =
        crate::commands::optimizer::env_loader::load_optimizer_env(project_root, &runtime)?;
    let base_envs: Vec<(OsString, OsString)> = optimizer_envs
        .into_iter()
        .map(|(k, v)| (OsString::from(k), OsString::from(v)))
        .collect();

    match tokens.as_slice() {
        // Bun runtime (allowed in DevServer scope with project script)
        // Runtime Bun (cho phép trong scope DevServer với script của project)
        // SAFETY: Validate args - no --eval, no arbitrary code execution
        ["bun", "run", rest @ ..] => {
            validate_runtime_args("bun", rest)?;
            Ok(DevLaunch {
                program: PathBuf::from("bun"),
                args: {
                    let mut args = vec![OsString::from("run")];
                    args.extend(rest.iter().map(OsString::from));
                    args
                },
                envs: base_envs.clone(),
            })
        }
        ["bun", rest @ ..] => {
            validate_runtime_args("bun", rest)?;
            Ok(DevLaunch {
                program: PathBuf::from("bun"),
                args: rest.iter().map(OsString::from).collect(),
                envs: base_envs.clone(),
            })
        }
        // Deno runtime (allowed in DevServer scope with project script)
        // Runtime Deno (cho phép trong scope DevServer với script của project)
        // SAFETY: Validate args - no --eval, restrict dangerous permissions
        ["deno", "run", rest @ ..] => {
            validate_runtime_args("deno", rest)?;
            Ok(DevLaunch {
                program: PathBuf::from("deno"),
                args: {
                    let mut args = vec![OsString::from("run")];
                    args.extend(rest.iter().map(OsString::from));
                    args
                },
                envs: base_envs.clone(),
            })
        }
        ["deno", "task", rest @ ..] => {
            validate_runtime_args("deno", rest)?;
            Ok(DevLaunch {
                program: PathBuf::from("deno"),
                args: {
                    let mut args = vec![OsString::from("task")];
                    args.extend(rest.iter().map(OsString::from));
                    args
                },
                envs: base_envs.clone(),
            })
        }
        ["vite"] | ["vite", "dev"] => {
            let mut args = Vec::new();
            append_dev_endpoint_args(&mut args, "--host", "--port", host, port);
            // The vite program is NEVER executed — run_single_dev_target
            // routes vite launches to MgDevServer (native Rust). Resolving
            // the binary here would demand node_modules for a file mgc
            // never runs, so the program stays an unresolved marker.
            // (Program vite không bao giờ chạy — MgDevServer chạy thay.)
            Ok(DevLaunch {
                program: PathBuf::from("vite"),
                args,
                envs: base_envs,
            })
        }
        ["vite", rest @ ..] => {
            let mut args: Vec<OsString> = rest.iter().map(OsString::from).collect();
            append_dev_endpoint_args(&mut args, "--host", "--port", host, port);
            Ok(DevLaunch {
                program: PathBuf::from("vite"),
                args,
                envs: base_envs,
            })
        }
        ["next", "dev"] => {
            let mut args = vec![OsString::from("dev")];
            append_dev_endpoint_args(&mut args, "-H", "-p", host, port);
            Ok(DevLaunch {
                program: resolve_local_bin(project_root, "next")?,
                args,
                envs: base_envs,
            })
        }
        ["next", "dev", rest @ ..] => {
            let mut args = vec![OsString::from("dev")];
            args.extend(rest.iter().map(OsString::from));
            append_dev_endpoint_args(&mut args, "-H", "-p", host, port);
            Ok(DevLaunch {
                program: resolve_local_bin(project_root, "next")?,
                args,
                envs: base_envs,
            })
        }
        ["nuxt", "dev"] | ["nuxt", "dev", "--host"] => {
            let mut args = vec![OsString::from("dev")];
            append_dev_endpoint_args(&mut args, "--host", "--port", host, port);
            let mut envs = base_envs.clone();
            envs.extend(vec![
                (
                    OsString::from("NUXT_TELEMETRY_DISABLED"),
                    OsString::from("1"),
                ),
                (
                    OsString::from("NUXT_TELEMETRY_CONSENT"),
                    OsString::from("0"),
                ),
            ]);
            Ok(DevLaunch {
                program: resolve_local_bin(project_root, "nuxt")?,
                args,
                envs,
            })
        }
        ["nuxt", "dev", rest @ ..] => {
            let mut args = vec![OsString::from("dev")];
            args.extend(rest.iter().map(OsString::from));
            append_dev_endpoint_args(&mut args, "--host", "--port", host, port);
            let mut envs = base_envs.clone();
            envs.extend(vec![
                (
                    OsString::from("NUXT_TELEMETRY_DISABLED"),
                    OsString::from("1"),
                ),
                (
                    OsString::from("NUXT_TELEMETRY_CONSENT"),
                    OsString::from("0"),
                ),
            ]);
            Ok(DevLaunch {
                program: resolve_local_bin(project_root, "nuxt")?,
                args,
                envs,
            })
        }
        ["astro", "dev"] => {
            let mut args = vec![OsString::from("dev")];
            append_dev_endpoint_args(&mut args, "--host", "--port", host, port);
            Ok(DevLaunch {
                program: resolve_local_bin(project_root, "astro")?,
                args,
                envs: base_envs.clone(),
            })
        }
        ["astro", "dev", rest @ ..] => {
            let mut args = vec![OsString::from("dev")];
            args.extend(rest.iter().map(OsString::from));
            append_dev_endpoint_args(&mut args, "--host", "--port", host, port);
            Ok(DevLaunch {
                program: resolve_local_bin(project_root, "astro")?,
                args,
                envs: base_envs.clone(),
            })
        }
        ["remix", "vite:dev"] => {
            let mut args = vec![OsString::from("vite:dev")];
            append_dev_endpoint_args(&mut args, "--host", "--port", host, port);
            Ok(DevLaunch {
                program: resolve_local_bin(project_root, "remix")?,
                args,
                envs: base_envs.clone(),
            })
        }
        ["remix", "vite:dev", rest @ ..] => {
            let mut args = vec![OsString::from("vite:dev")];
            args.extend(rest.iter().map(OsString::from));
            append_dev_endpoint_args(&mut args, "--host", "--port", host, port);
            Ok(DevLaunch {
                program: resolve_local_bin(project_root, "remix")?,
                args,
                envs: base_envs.clone(),
            })
        }
        ["ng", "serve"] => {
            let mut args = vec![OsString::from("serve")];
            append_dev_endpoint_args(&mut args, "--host", "--port", host, port);
            let mut envs = base_envs.clone();
            envs.extend(vec![
                (OsString::from("NG_CLI_ANALYTICS"), OsString::from("false")),
                (OsString::from("CI"), OsString::from("1")),
            ]);
            Ok(DevLaunch {
                program: resolve_local_bin(project_root, "ng")?,
                args,
                envs,
            })
        }
        ["ng", "serve", rest @ ..] => {
            let mut args = vec![OsString::from("serve")];
            args.extend(rest.iter().map(OsString::from));
            append_dev_endpoint_args(&mut args, "--host", "--port", host, port);
            let mut envs = base_envs.clone();
            envs.extend(vec![
                (OsString::from("NG_CLI_ANALYTICS"), OsString::from("false")),
                (OsString::from("CI"), OsString::from("1")),
            ]);
            Ok(DevLaunch {
                program: resolve_local_bin(project_root, "ng")?,
                args,
                envs,
            })
        }
        ["node", rest @ ..] => Ok(DevLaunch {
            program: PathBuf::from("node"),
            args: rest.iter().map(OsString::from).collect(),
            envs: base_envs.clone(),
        }),
        ["tsx", rest @ ..] => Ok(DevLaunch {
            program: resolve_local_bin(project_root, "tsx")?,
            args: rest.iter().map(OsString::from).collect(),
            envs: base_envs,
        }),
        _ => Err(crate::error::web_unsupported_dev_script(
            &script,
            project_root,
        )),
    }
}

fn reject_external_package_manager_script(
    script: &str,
    manifest_path: &Path,
    compat_runner: Option<&str>,
) -> Result<()> {
    // P0-1 fix (2026-09-10 audit): carve-out "bun run/deno run/deno task"
    // vô điều kiện đã BỎ — runtime đối thủ giờ qua gate_runtime_spawn với
    // CompatMode thật (native từ chối, compat tường minh mới mở). Ở đây
    // chỉ chặn PM ngoài (npm/npx/pnpm/yarn/bunx) như trước.
    // Chặn PM ngoài trong script (install là việc của resolver mgc).
    // Spec rev4 §7/§9: the declared `--compat-runner` manager is exempt from
    // the script-token check; its subtree runs compat-audit downstream.
    // (Spec rev4 §7/§9: manager `--compat-runner` đã khai báo được miễn
    // check token script; subtree của nó chịu compat-audit phía sau.)
    if let Some(pm) = mgc_exec::allowlist::find_forbidden_tool_in_script(script)
        && !mgc_exec::prelude::compat_runner_allows(pm, compat_runner)
    {
        // "bun run x" ở đây là runtime usage — gate_runtime_spawn đã xử lý
        // bun ở trên; chỉ từ chối nếu token bun là PM thật (bun install).
        let tokens: Vec<&str> = script.split_whitespace().collect();
        let is_runtime_usage = matches!(
            tokens.as_slice(),
            ["bun", "run", ..] | ["deno", "run", ..] | ["deno", "task", ..]
        );
        if !is_runtime_usage {
            return Err(crate::error::web_forbidden_pm(script, manifest_path, pm));
        }
    }
    Ok(())
}

fn has_backend_manifest(dir: &Path) -> bool {
    dir.join("package.json").exists()
        || dir.join("go.mod").exists()
        || dir.join("Cargo.toml").exists()
        || dir.join("manage.py").exists()
        || dir.join("main.py").exists()
        || dir.join("src/main.py").exists()
        || dir.join("pom.xml").exists()
        || dir.join("artisan").exists()
        || dir.join("composer.json").exists()
}

fn infer_native_dev_launch(
    project_root: &Path,
    host: Option<String>,
    port: Option<u16>,
) -> Result<DevLaunch> {
    let host_str = host.as_deref().unwrap_or("localhost");
    if project_root.join("go.mod").exists() {
        let go_dir = if project_root.join("cmd/server").exists() {
            OsString::from("./cmd/server")
        } else {
            OsString::from(".")
        };
        return Ok(DevLaunch {
            program: PathBuf::from("go"),
            args: vec![OsString::from("run"), go_dir],
            envs: env_host_port_pairs(host, port),
        });
    }

    if project_root.join("manage.py").exists() {
        let bind = format!("{host_str}:{}", port.unwrap_or(3415));
        let python = native_python_program(project_root);
        return Ok(DevLaunch {
            program: python,
            args: vec![
                OsString::from("manage.py"),
                OsString::from("runserver"),
                OsString::from(bind),
            ],
            envs: env_host_port_pairs(host, port),
        });
    }

    if project_root.join("main.py").exists() {
        let python = native_python_program(project_root);
        let fastapi_module = native_fastapi_asgi_module(project_root)?;
        let args = if let Some(module) = fastapi_module {
            vec![
                OsString::from("-m"),
                OsString::from("uvicorn"),
                OsString::from(module),
                OsString::from("--host"),
                OsString::from(host_str),
                OsString::from("--port"),
                OsString::from(port.unwrap_or(3415).to_string()),
            ]
        } else {
            vec![OsString::from("main.py")]
        };
        return Ok(DevLaunch {
            program: python,
            args,
            envs: env_host_port_pairs(host, port),
        });
    }

    if project_root.join("src/main.py").exists() {
        let python = native_python_program(project_root);
        let fastapi_module = native_fastapi_asgi_module(project_root)?;
        let args = if let Some(module) = fastapi_module {
            vec![
                OsString::from("-m"),
                OsString::from("uvicorn"),
                OsString::from(module),
                OsString::from("--host"),
                OsString::from(host_str),
                OsString::from("--port"),
                OsString::from(port.unwrap_or(3415).to_string()),
            ]
        } else {
            vec![OsString::from("-m"), OsString::from("src.main")]
        };
        return Ok(DevLaunch {
            program: python,
            args,
            envs: env_host_port_pairs(host, port),
        });
    }

    if project_root.join("Cargo.toml").exists() && project_root.join("src/main.rs").exists() {
        #[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
        {
            return Ok(DevLaunch {
                program: PathBuf::from("cargo"),
                args: native_cargo_run_args(project_root)?,
                envs: env_host_port_pairs(host, port),
            });
        }
        #[cfg(not(any(feature = "lib", feature = "web-backend-runtime")))]
        bail!("Rust Web dev requires a CLI build with the shared language adapter enabled");
    }

    if project_root.join("artisan").exists() {
        return Ok(DevLaunch {
            program: PathBuf::from("php"),
            args: vec![
                OsString::from("artisan"),
                OsString::from("serve"),
                OsString::from(format!("--host={host_str}")),
                OsString::from(format!("--port={}", port.unwrap_or(8000))),
            ],
            envs: env_host_port_pairs(host, port),
        });
    }

    if project_root.join("composer.json").exists()
        && project_root.join("public/index.php").exists()
        && project_root.join("bin/console").exists()
    {
        return Ok(DevLaunch {
            program: PathBuf::from("php"),
            args: vec![
                OsString::from("-S"),
                OsString::from(format!("{host_str}:{}", port.unwrap_or(8000))),
                OsString::from("-t"),
                OsString::from("public"),
                OsString::from("public/index.php"),
            ],
            envs: env_host_port_pairs(host, port),
        });
    }

    if project_root.join("composer.json").exists() && project_root.join("public/index.php").exists()
    {
        return Ok(DevLaunch {
            program: PathBuf::from("php"),
            args: vec![
                OsString::from("-S"),
                OsString::from(format!("{host_str}:{}", port.unwrap_or(8000))),
                OsString::from("-t"),
                OsString::from("public"),
                OsString::from("public/index.php"),
            ],
            envs: env_host_port_pairs(host, port),
        });
    }

    if project_root.join("pom.xml").exists() {
        let pom = std::fs::read_to_string(project_root.join("pom.xml")).unwrap_or_default();
        if pom.contains("quarkus.platform") || pom.contains("io.quarkus") {
            let mut args = vec![
                OsString::from("quarkus:dev"),
                OsString::from(format!("-Dquarkus.http.host={host_str}")),
                OsString::from("-Dquarkus.analytics.disabled=true"),
            ];
            if let Some(port) = port {
                args.push(OsString::from(format!("-Dquarkus.http.port={port}")));
            }
            return Ok(DevLaunch {
                program: PathBuf::from("mvn"),
                args,
                envs: env_host_port_pairs(host, port),
            });
        }

        let host_arg = format!("--server.address={host_str}");
        let mut args = vec![OsString::from("spring-boot:run")];
        if let Some(port) = port {
            args.push(OsString::from(format!(
                "-Dspring-boot.run.arguments=--server.port={port} {host_arg}"
            )));
        } else {
            args.push(OsString::from(format!(
                "-Dspring-boot.run.arguments={host_arg}"
            )));
        }
        return Ok(DevLaunch {
            program: PathBuf::from("mvn"),
            args,
            envs: env_host_port_pairs(host, port),
        });
    }

    Err(crate::error::web_no_dev_entrypoint(project_root))
}

/// Start Rust Web backends from the Cargo projection and sources verified by MGC.
/// Chạy backend Rust Web bằng lock/source Cargo đã được MGC xác minh.
#[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
fn native_cargo_run_args(project_root: &Path) -> Result<Vec<OsString>> {
    let source_root =
        mgc_lib_adapter::install::cargo_compat::validate_build_for_core(project_root, "web")?;
    cargo_run_args_from_managed_source(source_root.as_deref())
}

fn cargo_run_args_from_managed_source(source_root: Option<&Path>) -> Result<Vec<OsString>> {
    let mut args = Vec::new();
    if let Some(source_root) = source_root {
        let source_path = source_root.to_str().ok_or_else(|| {
            anyhow::anyhow!("MagiCore Cargo source path must be valid UTF-8 on this platform")
        })?;
        let source_config = format!(
            "source.mgc-managed.directory={}",
            toml::Value::String(source_path.to_string())
        );
        args.extend([
            OsString::from("--config"),
            OsString::from("source.crates-io.replace-with=\"mgc-managed\""),
            OsString::from("--config"),
            OsString::from(source_config),
        ]);
    }
    args.extend([
        OsString::from("run"),
        OsString::from("--locked"),
        OsString::from("--offline"),
    ]);
    Ok(args)
}

/// Resolve the ASGI module only when the project explicitly declares FastAPI.
/// Chỉ suy ra module ASGI khi cấu hình project khai báo rõ FastAPI.
fn native_fastapi_asgi_module(project_root: &Path) -> Result<Option<&'static str>> {
    let configured =
        mgc_config::project::ProjectConfig::load(project_root)?.is_some_and(|config| {
            config.ecosystem.eq_ignore_ascii_case("web")
                && config
                    .frameworks
                    .iter()
                    .any(|framework| framework.eq_ignore_ascii_case("fastapi"))
        });
    if !configured {
        return Ok(None);
    }

    if project_root.join("main.py").is_file() {
        Ok(Some("main:app"))
    } else if project_root.join("src/main.py").is_file() {
        Ok(Some("src.main:app"))
    } else {
        Ok(None)
    }
}

fn env_host_port_pairs(host: Option<String>, port: Option<u16>) -> Vec<(OsString, OsString)> {
    let mut envs = Vec::new();
    if let Some(host) = host {
        envs.push((OsString::from("HOST"), OsString::from(host)));
    }
    if let Some(port) = port {
        let port = port.to_string();
        envs.push((OsString::from("PORT"), OsString::from(port.clone())));
        envs.push((OsString::from("SERVER_PORT"), OsString::from(port.clone())));
        envs.push((OsString::from("QUARKUS_HTTP_PORT"), OsString::from(port)));
    }
    envs
}

fn native_python_program(project_root: &Path) -> PathBuf {
    let venv_python = native_venv_executable(project_root, "python");
    if venv_python.exists() {
        return venv_python;
    }

    #[cfg(windows)]
    {
        return PathBuf::from("python");
    }

    #[allow(unreachable_code)]
    PathBuf::from("python3")
}

fn native_venv_executable(project_root: &Path, bin_name: &str) -> PathBuf {
    #[cfg(windows)]
    {
        return project_root
            .join(".venv")
            .join("Scripts")
            .join(format!("{bin_name}.exe"));
    }

    #[allow(unreachable_code)]
    project_root.join(".venv").join("bin").join(bin_name)
}

/// Which compat tool owns a non-package.json monorepo member (pure —
/// unit-tested)? The mapping is HARD: each member kind opens with exactly
/// the tool that will be spawned, never a sibling (`pip3`/`uv` do NOT
/// open the pip branch — the spawn runs `pip`, so only `pip` opens it).
/// (Map cứng member → tool: chỉ đúng tool được spawn mới mở được nhánh.)
fn non_native_member_ecosystem(project_root: &Path) -> Option<&'static str> {
    if project_root.join("go.mod").exists() {
        Some("go")
    } else if project_root.join("requirements.txt").exists() {
        Some("python")
    } else if project_root.join("Cargo.toml").exists() {
        Some("rust")
    } else if project_root.join("pom.xml").exists() {
        Some("java")
    } else if project_root.join("composer.json").exists() || project_root.join("artisan").exists() {
        Some("php")
    } else {
        None
    }
}

/// Fail closed for non-JS package manifests until MGC owns their lifecycle.
/// Từ chối manifest ngoài JS đến khi MGC sở hữu lifecycle tương ứng.
fn compat_gate_decision(project_root: &Path) -> Result<&'static str> {
    let Some(ecosystem) = non_native_member_ecosystem(project_root) else {
        return Err(crate::error::web_no_install_flow(project_root));
    };
    Err(crate::error::native_dependency_engine_unavailable(
        "web monorepo member",
        ecosystem,
        "install",
    ))
}

fn compat_install_target(
    project_root: &Path,
    _compat: &crate::commands::compat::CompatMode,
) -> Result<()> {
    compat_gate_decision(project_root).map(|_| ())
}

fn native_install_env(project_root: &Path, program: &str) -> Result<Vec<(String, String)>> {
    let mut env = Vec::new();
    // Hermetic HOME: clean-env spawns wipe $HOME, but some tools
    // hard-require it (composer refuses to run without HOME or
    // COMPOSER_HOME). Point it at project-scoped cache so cache writes
    // stay inside .magicore instead of the user's real home.
    // (HOME cách ly: tool đòi $HOME cũng chỉ thấy cache của project.)
    let cache_home = project_root.join(".magicore").join("cache").join("home");
    std::fs::create_dir_all(&cache_home)?;
    env.push(("HOME".to_string(), cache_home.display().to_string()));
    if program == "composer" {
        let composer_home = project_root
            .join(".magicore")
            .join("cache")
            .join("composer");
        let composer_cache = composer_home.join("cache");
        std::fs::create_dir_all(&composer_cache)?;
        env.push((
            "COMPOSER_HOME".to_string(),
            composer_home.display().to_string(),
        ));
        env.push((
            "COMPOSER_CACHE_DIR".to_string(),
            composer_cache.display().to_string(),
        ));
    }
    if program == "go" {
        let paths = mgc_platform::paths::GlobalPaths::new()?;
        let mod_cache = paths.store.join("go");
        let build_cache = paths.cache.join("go").join("build");
        std::fs::create_dir_all(&mod_cache)?;
        std::fs::create_dir_all(&build_cache)?;
        env.extend(go_native_runtime_env(&mod_cache, &build_cache));
    }
    if program == "cargo" {
        env.push(("CARGO_NET_OFFLINE".to_string(), "true".to_string()));
    }
    Ok(env)
}

/// Keep Go dev execution on the same verified module cache used by native install.
/// Buộc `mgc dev` dùng module cache đã cài/xác minh bởi MGC và chặn tải ngoài.
fn go_native_runtime_env(mod_cache: &Path, build_cache: &Path) -> Vec<(String, String)> {
    vec![
        ("GOMODCACHE".to_string(), mod_cache.display().to_string()),
        ("GOCACHE".to_string(), build_cache.display().to_string()),
        ("GOPROXY".to_string(), "off".to_string()),
        ("GOSUMDB".to_string(), "off".to_string()),
        ("GOTOOLCHAIN".to_string(), "local".to_string()),
    ]
}

async fn run_single_dev_target(
    target: &DevTarget,
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
) -> Result<()> {
    let launch = build_dev_launch(
        &target.dir,
        target.script_name,
        target.host.clone(),
        target.port,
        compat,
        compat_runner,
    )?;

    if launch.program.to_string_lossy().ends_with("vite") {
        info(&format!(
            "Starting MgDevServer (Native Rust) in {}",
            target.dir.display()
        ));
    } else {
        info(&format!(
            "Starting web dev server in {}",
            target.dir.display()
        ));
        info(&format!("  {}", launch.describe()));
    }

    // Lifecycle hooks (Spec rev4 §16.1 — decided): predev aborts on failure,
    // postdev always runs once after the server stops.
    // (Hook lifecycle (Spec rev4 §16.1 — đã chốt): predev hỏng thì dừng,
    // postdev luôn chạy một lần sau khi server dừng.)
    run_dev_hook(&target.dir, "predev", compat, compat_runner)?;
    let result = if launch.program.to_string_lossy().ends_with("vite") {
        let entry = if target.dir.join("src/main.tsx").exists() {
            target.dir.join("src/main.tsx")
        } else if target.dir.join("src/main.ts").exists() {
            target.dir.join("src/main.ts")
        } else {
            target.dir.join("src/index.tsx")
        };

        let config = crate::bundler::dev_server::DevServerConfig {
            root: target.dir.clone(),
            entry,
            host: target
                .host
                .clone()
                .unwrap_or_else(|| "localhost".to_string()),
            port: target.port.unwrap_or(4315),
        };
        mgc_exec::prelude::ensure_signal_handlers();
        let server = crate::bundler::dev_server::MgDevServer::new(config);
        let cancel_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
        tokio::select! {
            result = server.serve() => result,
            stop = wait_for_dev_stop(cancel_flag) => stop,
        }
    } else {
        run_dev_launch_with_guard(target, &launch, compat, compat_runner, None)
    };
    if mgc_exec::prelude::shutdown_requested() {
        mgc_exec::prelude::clear_shutdown();
    }
    let post = run_dev_hook(&target.dir, "postdev", compat, compat_runner);
    if let Err(post_err) = post {
        mgc_ui::warning(&format!("post hook 'postdev' failed: {post_err:#}"));
    }
    result
}

/// Run a dev lifecycle hook (`predev`/`postdev`) if the project defines it.
/// Missing hooks are a no-op; hook failures propagate to the caller.
/// (Chạy hook lifecycle dev (`predev`/`postdev`) nếu project định nghĩa.
/// Hook vắng là no-op; hook hỏng lan tới caller.)
fn run_dev_hook(
    dir: &std::path::Path,
    hook: &str,
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
) -> Result<()> {
    // Reuse the single task-script implementation (same precedence, gates
    // and compat wiring as `mgc run`) — one behavior, not two.
    // (Dùng chung implementation task-script (cùng precedence, gate và
    // compat như `mgc run`) — một hành vi, không hai.)
    if crate::commands::run::resolve_task_script(dir, hook)?.is_none() {
        return Ok(());
    }
    crate::commands::run::run_task_script(dir, hook, &[], compat, compat_runner, "dev")
}

async fn run_multi_dev_targets(
    targets: &[DevTarget],
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
) -> Result<()> {
    if targets.is_empty() {
        return Ok(());
    }

    // Resolve every launch before lifecycle hooks or child processes begin.
    // (Resolve mọi launch trước hook lifecycle hoặc khi process con bắt đầu.)
    let launches = targets
        .iter()
        .map(|target| {
            build_dev_launch(
                &target.dir,
                target.script_name,
                target.host.clone(),
                target.port,
                compat,
                compat_runner,
            )
            .map(|launch| (target.clone(), launch))
        })
        .collect::<Result<Vec<_>>>()?;

    for target in targets {
        run_dev_hook(&target.dir, "predev", compat, compat_runner)?;
    }

    let cancel_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
    mgc_exec::prelude::ensure_signal_handlers();
    let mut tasks = tokio::task::JoinSet::<Result<()>>::new();

    for (target, launch) in launches {
        if launch.program.to_string_lossy().ends_with("vite") {
            info(&format!(
                "Starting MgDevServer (Native Rust) for {} in {}",
                target.role,
                target.dir.display()
            ));

            let entry = if target.dir.join("src/main.tsx").exists() {
                target.dir.join("src/main.tsx")
            } else if target.dir.join("src/main.ts").exists() {
                target.dir.join("src/main.ts")
            } else {
                target.dir.join("src/index.tsx")
            };

            let config = crate::bundler::dev_server::DevServerConfig {
                root: target.dir.clone(),
                entry,
                host: target
                    .host
                    .clone()
                    .unwrap_or_else(|| "localhost".to_string()),
                port: target.port.unwrap_or(4315),
            };

            let server = crate::bundler::dev_server::MgDevServer::new(config);
            let cancel_flag = Arc::clone(&cancel_flag);
            tasks.spawn(async move {
                tokio::select! {
                    result = server.serve() => result,
                    stop = wait_for_dev_stop(cancel_flag) => stop,
                }
            });
            continue;
        }

        info(&format!(
            "Starting {} dev server in {}",
            target.role,
            target.dir.display()
        ));
        info(&format!("  {}", launch.describe()));
        tasks.spawn({
            let compat = compat.clone();
            let compat_runner = compat_runner.map(str::to_string);
            let cancel_flag = Arc::clone(&cancel_flag);
            async move {
                tokio::task::spawn_blocking(move || {
                    run_dev_launch_with_guard(
                        &target,
                        &launch,
                        &compat,
                        compat_runner.as_deref(),
                        Some(cancel_flag),
                    )
                })
                .await?
            }
        });
    }

    let result = supervise_dev_tasks(&mut tasks, cancel_flag).await;
    if mgc_exec::prelude::shutdown_requested() {
        mgc_exec::prelude::clear_shutdown();
    }
    for target in targets {
        if let Err(post_err) = run_dev_hook(&target.dir, "postdev", compat, compat_runner) {
            mgc_ui::warning(&format!("post hook 'postdev' failed: {post_err:#}"));
        }
    }
    result
}

const DEV_CANCEL_POLL_INTERVAL_MS: u64 = 25;

/// Wait for either a process-wide signal or sibling cancellation.
/// Chờ tín hiệu dừng toàn process hoặc yêu cầu hủy từ target cùng nhóm.
async fn wait_for_dev_stop(cancel_flag: Arc<std::sync::atomic::AtomicBool>) -> Result<()> {
    loop {
        if mgc_exec::prelude::shutdown_requested() {
            return Err(crate::error::web_dev_interrupted());
        }
        if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(crate::error::web_dev_target_cancelled());
        }
        tokio::time::sleep(std::time::Duration::from_millis(
            DEV_CANCEL_POLL_INTERVAL_MS,
        ))
        .await;
    }
}

/// Stop every sibling after the first target finishes, then reap all tasks.
/// Dừng target cùng nhóm khi target đầu tiên kết thúc, sau đó thu hồi hết task.
async fn supervise_dev_tasks(
    tasks: &mut tokio::task::JoinSet<Result<()>>,
    cancel_flag: Arc<std::sync::atomic::AtomicBool>,
) -> Result<()> {
    let Some(first) = tasks.join_next().await else {
        return Ok(());
    };

    cancel_flag.store(true, std::sync::atomic::Ordering::SeqCst);
    let mut first_result = match first {
        Ok(result) => result,
        Err(error) => Err(error.into()),
    };
    let mut reap_error = None;
    while let Some(result) = tasks.join_next().await {
        if let Err(error) = result
            && reap_error.is_none()
        {
            reap_error = Some(anyhow::Error::from(error));
        }
    }

    if first_result.is_ok()
        && let Some(error) = reap_error
    {
        first_result = Err(error);
    }
    first_result
}

fn run_dev_launch_with_guard(
    target: &DevTarget,
    launch: &DevLaunch,
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
    cancel_flag: Option<Arc<std::sync::atomic::AtomicBool>>,
) -> Result<()> {
    let local_bin = target.dir.join("node_modules").join(".bin");
    let mut env = vec![(
        "PATH".to_string(),
        prepend_path(&local_bin)?.to_string_lossy().to_string(),
    )];
    env.extend(native_runtime_env(&target.dir, &launch.program)?);
    env.extend(launch.envs.iter().map(|(key, value)| {
        (
            key.to_string_lossy().to_string(),
            value.to_string_lossy().to_string(),
        )
    }));
    if let Some((key, value)) = native_astro_update_check_env(&launch.program, compat) {
        env.push((key, value));
    }
    if let Some(port) = target.port {
        env.push(("PORT".to_string(), port.to_string()));
    }

    let args = launch
        .args
        .iter()
        .map(|arg| arg.to_string_lossy().to_string())
        .collect::<Vec<_>>();

    // SAFETY: Enable audit log for dev server execution
    // AN TOÀN: Bật audit log cho dev server execution
    let audit_log = target.dir.join(".magicore").join("exec.log");
    if let Some(parent) = audit_log.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // P0-1: compat lane truyền runtime đã chọn xuống mgc-exec để
    // exemption áp ĐÚNG runtime đã qua cổng (bun/deno compat spawn).
    let compat_runtime = match compat {
        crate::commands::compat::CompatMode::Native => None,
        crate::commands::compat::CompatMode::Explicit(runtime) => Some(runtime.clone()),
    };
    // Exact-manifest lane context for this operation (exact-match only).
    // (Ngữ cảnh lane cho manifest exception của operation này (chỉ khớp chính xác).)
    let (policy_manifest, policy_context, policy_profile) =
        crate::commands::policy::load_policy_for_run(&target.dir, "dev");
    let opts = mgc_exec::prelude::ExecOptions {
        cwd: Some(target.dir.clone()),
        env,
        clean_env: true,
        disable_timeout: true,
        execution_scope: Some(mgc_exec::prelude::ExecutionScope::DevServer),
        log_path: Some(audit_log),
        compat_runtime,
        compat_runner: compat_runner.map(str::to_string),
        policy_manifest,
        policy_context,
        policy_profile,
        ..Default::default()
    };

    let execution = match cancel_flag.as_deref() {
        Some(cancel_flag) if launch.program.components().count() > 1 => {
            mgc_exec::prelude::run_project_binary_inherited_cancellable(
                &launch.program,
                &args,
                &opts,
                cancel_flag,
            )
        }
        Some(cancel_flag) => mgc_exec::prelude::run_inherited_cancellable(
            &launch.program.to_string_lossy(),
            &args,
            &opts,
            cancel_flag,
        ),
        None if launch.program.components().count() > 1 => {
            mgc_exec::prelude::run_project_binary_inherited(&launch.program, &args, &opts)
        }
        None => mgc_exec::prelude::run_inherited(&launch.program.to_string_lossy(), &args, &opts),
    };
    execution.with_context(|| format!("failed to start '{}'", launch.program.to_string_lossy()))?;
    Ok(())
}

fn native_runtime_env(project_root: &Path, program: &Path) -> Result<Vec<(String, String)>> {
    let Some(name) = program.file_name().and_then(|name| name.to_str()) else {
        return Ok(Vec::new());
    };
    let mut env = native_install_env(project_root, name)?;
    if is_python_interpreter_name(name) {
        crate::commands::python_runtime::extend_native_python_env(&mut env, project_root, false)?;
    }
    Ok(env)
}

fn is_python_interpreter_name(name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    let stem = name.strip_suffix(".exe").unwrap_or(&name);
    if stem == "python" {
        return true;
    }

    let Some(version) = stem.strip_prefix("python") else {
        return false;
    };
    let mut previous_was_dot = true;
    let mut saw_digit = false;
    for character in version.chars() {
        if character.is_ascii_digit() {
            previous_was_dot = false;
            saw_digit = true;
        } else if character == '.' && !previous_was_dot {
            previous_was_dot = true;
        } else {
            return false;
        }
    }
    saw_digit && !previous_was_dot
}

fn native_astro_update_check_env(
    program: &Path,
    compat: &crate::commands::compat::CompatMode,
) -> Option<(String, String)> {
    let is_astro = program
        .file_stem()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.eq_ignore_ascii_case("astro"));
    (is_astro && matches!(compat, crate::commands::compat::CompatMode::Native))
        .then(|| ("ASTRO_DISABLE_UPDATE_CHECK".to_string(), "true".to_string()))
}

fn resolve_local_bin(project_root: &Path, bin_name: &str) -> Result<PathBuf> {
    let bin_dir = project_root.join("node_modules").join(".bin");
    let candidates = local_bin_candidates(&bin_dir, bin_name);
    if let Some(path) = candidates.into_iter().find(|path| path.exists()) {
        return Ok(path);
    }

    Err(crate::error::web_missing_executable(
        bin_name,
        install_hint_command(),
        project_root,
    ))
}

fn local_bin_candidates(bin_dir: &Path, bin_name: &str) -> Vec<PathBuf> {
    #[cfg(not(windows))]
    {
        vec![bin_dir.join(bin_name)]
    }
    #[cfg(windows)]
    {
        let mut candidates = vec![bin_dir.join(bin_name)];
        candidates.push(bin_dir.join(format!("{bin_name}.cmd")));
        candidates.push(bin_dir.join(format!("{bin_name}.exe")));
        candidates
    }
}

fn prepend_path(local_bin: &Path) -> Result<OsString> {
    let mut paths = vec![local_bin.to_path_buf()];
    if let Some(existing) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&existing));
    }
    Ok(std::env::join_paths(paths)?)
}

// ── Scaffold (create) ────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FrameworkRequest {
    pub raw: String,
    pub normalized: String,
    pub version: Option<String>,
    pub dist_tag: Option<String>,
}

pub async fn run_create_with_options(
    framework: &str,
    project_name: &str,
    flags: Option<ScaffoldFlags>,
) -> Result<()> {
    let mut flags = flags.unwrap_or_default();

    if let Some(preset_name) = &flags.preset.clone() {
        apply_preset(preset_name, &mut flags);
    }

    // Phase 4: Parse scaffold spec sớm với typo detection
    use crate::scaffold::spec::{CoreKind, parse_scaffold_spec};
    let spec = parse_scaffold_spec(CoreKind::Web, framework)
        .map_err(|e| anyhow::anyhow!("Invalid framework specification '{}': {}", framework, e))?;

    apply_framework_alias_defaults(&spec.name, &mut flags);

    // Use normalized name cho resolve
    let fe_framework = resolve_framework(Some(&spec.name), &flags)?;
    enforce_framework_language_defaults(&fe_framework, &mut flags);
    validate_flags(&flags, &fe_framework)?;

    info(&format!(
        "Creating new web project '{}' with {}",
        project_name, fe_framework
    ));

    let config = build_web_config(&fe_framework, project_name, &flags)?;

    // Registry-first preflight — ensure only layers this scaffold mode uses.
    // Kiểm đúng layer theo mode, không bắt backend/monorepo partials cho frontend.
    let frontend = framework_request_from_spec(&fe_framework, &spec.requested_ref);
    let rels = required_web_layers_for_config(&config, &frontend);

    // Phase 3: Typed resolution với MissingLayersReport thay vì warning spam
    use crate::scaffold::resolver::MissingLayersReport;
    let mut report = MissingLayersReport::new();

    for rel in &rels {
        match crate::commands::template::ensure_layer(rel).await {
            Ok(status) => {
                if !status.is_available() {
                    if web_layer_has_scaffold_fallback(&config, rel) {
                        report.add_optional(rel.clone());
                    } else {
                        report.add_required_failure(rel, format!("{status:?}"));
                    }
                }
            }
            Err(error) => {
                if error.is_availability_failure() && web_layer_has_scaffold_fallback(&config, rel)
                {
                    report.add_optional(rel.clone());
                } else {
                    report.add_required_failure(rel, error);
                }
            }
        }
    }

    // Fail early nếu có required layers missing
    if report.has_required_missing() {
        bail!(report.format_error("web", &frontend.normalized));
    }
    if !report.optional.is_empty() {
        mgc_ui::warning(&format!(
            "Using the built-in scaffold because template layers are unavailable: {}",
            report.optional.join(", ")
        ));
    }

    let project_dir = crate::scaffold::Scaffolder::scaffold(&config)?;

    let proj_config = mgc_config::project::ProjectConfig::from_scaffold(
        crate::scaffold::Scaffolder::display_name(&project_dir),
        "web",
        &config.sub_type,
        config.frameworks.clone(),
        config.template_dir.to_string_lossy(),
        config.features.clone(),
    );
    proj_config.save(&project_dir)?;

    // Write scaffold provenance (R10 - supply chain tracking)
    let provenance = crate::scaffold::provenance::ScaffoldProvenance::new(
        spec.name.clone(),
        "web".to_string(),
        match &spec.requested_ref {
            crate::scaffold::spec::ScaffoldRef::DistTag(t) => t.clone(),
            crate::scaffold::spec::ScaffoldRef::Version(v) => v.clone(),
            _ => "default".to_string(),
        },
        None, // Registry URL would come from ensure_layer results
        rels.clone(),
    );
    if let Err(e) = provenance.write(&project_dir) {
        mgc_ui::warning(&format!("Failed to write provenance: {}", e));
    }

    let be_name = detect_backend_framework(&flags);
    let backend = be_name
        .as_deref()
        .map(parse_framework_request)
        .or_else(|| fullstack_backend_framework(&frontend.normalized).map(parse_framework_request));
    enrich_web_project_manifest(&project_dir, &frontend, backend.as_ref(), &flags).await?;

    info(&format!("Project '{}' created!", project_dir.display()));
    info(&format!(
        "  cd {} && {}",
        project_name,
        install_hint_command()
    ));

    Ok(())
}

fn required_web_layers_for_config(
    config: &crate::wizard::engine::ScaffoldConfig,
    frontend: &FrameworkRequest,
) -> Vec<String> {
    let mut rels = vec!["web/shared/partials/base".to_string()];
    let primary = match config.sub_type.as_str() {
        "backend" => config.frameworks.last(),
        _ => config.frameworks.first(),
    }
    .cloned()
    .unwrap_or_else(|| frontend.normalized.clone());

    match config.sub_type.as_str() {
        "frontend" => {
            rels.push("web/shared/partials/frontend-foundation".to_string());
            rels.push("web/shared/partials/frontend-rust-ready".to_string());
            rels.push("web/shared/partials/frontend".to_string());
            if matches!(primary.as_str(), "react-vite" | "solidjs") {
                rels.push("web/shared/partials/frontend-common".to_string());
            }
            rels.push(format!("web/frontend/{primary}"));
        }
        "backend" => {
            rels.push("web/shared/partials/backend".to_string());
            if let Some(lang) = crate::scaffold::processor::infer_backend_language(&primary) {
                rels.push(format!("web/backend/{lang}/{primary}"));
            }
        }
        "fullstack" => {
            rels.push("web/shared/partials/fullstack".to_string());
            let bucket = if crate::scaffold::processor::is_all_in_one_fullstack(&primary) {
                "all-in-one"
            } else {
                "split"
            };
            rels.push(format!("web/fullstack/{bucket}/{primary}"));
        }
        "monorepo" => {
            rels.push("web/shared/partials/monorepo".to_string());
            rels.push("web/monorepo/base".to_string());
            rels.push(format!("web/monorepo/frontend/{primary}"));
            // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
            if let Some(backend) = config.frameworks.get(1)
                && let Some(lang) = crate::scaffold::processor::infer_backend_language(backend)
            {
                rels.push(format!("web/monorepo/backend/{lang}/{backend}"));
            }
        }
        _ => rels.push(format!("web/frontend/{primary}")),
    }

    rels
}

fn web_layer_has_scaffold_fallback(
    config: &crate::wizard::engine::ScaffoldConfig,
    _rel: &str,
) -> bool {
    if config.sub_type == "monorepo" {
        return false;
    }
    let framework = match config.sub_type.as_str() {
        "backend" => config.frameworks.last(),
        _ => config.frameworks.first(),
    }
    .map(String::as_str)
    .unwrap_or("");
    framework == "vanilla"
        || crate::scaffold::embedded_kernel::get_embedded_template("web", framework).is_some()
}

fn resolve_framework(pos: Option<&str>, flags: &ScaffoldFlags) -> Result<String> {
    if let Some(fw) = pos {
        return Ok(fw.to_string());
    }
    if flags.react {
        Ok("react".into())
    } else if flags.next {
        Ok("next".into())
    } else if flags.vue {
        Ok("vue".into())
    } else if flags.nuxt {
        Ok("nuxt".into())
    } else if flags.svelte {
        Ok("svelte".into())
    } else if flags.sveltekit {
        Ok("sveltekit".into())
    } else if flags.solid {
        Ok("solid".into())
    } else if flags.astro {
        Ok("astro".into())
    } else if flags.remix {
        Ok("remix".into())
    } else {
        Err(crate::error::no_framework_specified())
    }
}

fn detect_backend_framework(flags: &ScaffoldFlags) -> Option<String> {
    if flags.express {
        Some("express".into())
    } else if flags.fastify {
        Some("fastify".into())
    } else if flags.nestjs {
        Some("nestjs".into())
    } else if flags.hono {
        Some("hono".into())
    } else if flags.koa {
        Some("koa".into())
    } else if flags.trpc {
        Some("trpc".into())
    } else if flags.axum {
        Some("axum".into())
    } else if flags.actix_web {
        Some("actix-web".into())
    } else if flags.gin {
        Some("gin".into())
    } else if flags.echo {
        Some("echo".into())
    } else if flags.fiber {
        Some("fiber".into())
    } else if flags.fastapi {
        Some("fastapi".into())
    } else if flags.django {
        Some("django".into())
    } else if flags.flask {
        Some("flask".into())
    } else if flags.quarkus {
        Some("quarkus".into())
    } else if flags.symfony {
        Some("symfony".into())
    } else {
        None
    }
}

fn validate_flags(flags: &ScaffoldFlags, fe_framework: &str) -> Result<()> {
    if let Some(pm) = flags.pm.as_deref() {
        return Err(crate::error::pm_not_supported(pm));
    }

    if flags.ts && flags.js {
        return Err(crate::error::ts_js_exclusive());
    }

    let fe_count = [
        flags.react,
        flags.next,
        flags.vue,
        flags.nuxt,
        flags.svelte,
        flags.sveltekit,
        flags.solid,
        flags.astro,
        flags.remix,
    ]
    .iter()
    .filter(|&&b| b)
    .count();

    if fe_count > 1 {
        return Err(crate::error::multiple_frameworks());
    }

    if flags.pinia && fe_framework != "vue" && fe_framework != "nuxt" {
        return Err(crate::error::pinia_requires_vue());
    }

    if flags.shadcn && !flags.tailwindcss {
        // auto-enable tailwindcss (side-effect: user sees --tailwindcss in flags)
    }

    Ok(())
}

fn apply_preset(name: &str, flags: &mut ScaffoldFlags) {
    match name {
        "t3" => {
            flags.next = true;
            flags.ts = true;
            flags.tailwindcss = true;
            flags.trpc = true;
            flags.prisma = true;
            flags.zod = true;
            flags.nextauth = true;
        }
        "mern" => {
            flags.react = true;
            flags.express = true;
            flags.mongoose = true;
            flags.mongodb = true;
            flags.zod = true;
            flags.jwt = true;
        }
        "jamstack" => {
            flags.astro = true;
            flags.tailwindcss = true;
            flags.dotenv = true;
            flags.vercel = true;
        }
        "saas" => {
            flags.next = true;
            flags.tailwindcss = true;
            flags.shadcn = true;
            flags.prisma = true;
            flags.postgres = true;
            flags.clerk = true;
            flags.docker = true;
        }
        "mevn" => {
            flags.vue = true;
            flags.express = true;
            flags.mongoose = true;
            flags.mongodb = true;
            flags.zod = true;
        }
        _ => {}
    }
}

fn build_web_config(
    framework: &str,
    project_name: &str,
    flags: &ScaffoldFlags,
) -> Result<crate::wizard::engine::ScaffoldConfig> {
    let frontend = parse_framework_request(framework);
    // Fullstack combos (react-axum, vue-gin, ...) carry the backend in the
    // combined name; check the raw request so normalization to the shared
    // frontend leaf (react-vite) does not hide it.
    let raw_backend = fullstack_backend_framework(&frontend.raw)
        .or_else(|| fullstack_backend_framework(&frontend.normalized))
        .map(str::to_string);

    let mut config = if flags.monorepo {
        match detect_backend_framework(flags).or(raw_backend) {
            Some(be) => {
                let backend = parse_framework_request(&be);
                crate::wizard::engine::ScaffoldConfig {
                    core: "web".to_string(),
                    sub_type: "monorepo".to_string(),
                    frameworks: vec![frontend.normalized.clone(), backend.normalized],
                    project_name: project_name.to_string(),
                    features: vec![],
                    template_dir: std::path::PathBuf::new(),
                }
            }
            None => {
                info(
                    "--monorepo ignored: no backend framework specified (add --express, --fastify, etc.)",
                );
                crate::scaffold::Scaffolder::infer_web_create_config(
                    &frontend.normalized,
                    project_name,
                )?
            }
        }
    } else {
        crate::scaffold::Scaffolder::infer_web_create_config(&frontend.normalized, project_name)?
    };

    config.features = web_features(flags);

    if config.frameworks.len() >= 2 {
        let lang = &config.frameworks[0];
        if !config.features.contains(lang) {
            config.features.push(lang.clone());
        }
    }

    Ok(config)
}

fn enforce_framework_language_defaults(framework: &str, flags: &mut ScaffoldFlags) {
    let frontend = parse_framework_request(framework);
    let backend = detect_backend_framework(flags)
        .or_else(|| fullstack_backend_framework(&frontend.normalized).map(str::to_string));

    if frontend.normalized == "nestjs" || backend.as_deref() == Some("nestjs") {
        flags.ts = true;
        flags.js = false;
    }
}

fn web_features(flags: &ScaffoldFlags) -> Vec<String> {
    let mut features = Vec::new();

    // Language
    if flags.ts {
        features.push("typescript".into());
    }
    if flags.js {
        features.push("javascript".into());
    }

    // Styling
    if flags.tailwindcss || flags.shadcn || flags.daisyui {
        features.push("tailwindcss".into());
    }
    if flags.css_modules {
        features.push("css-modules".into());
    }
    if flags.styled_components {
        features.push("styled-components".into());
    }
    if flags.sass {
        features.push("sass".into());
    }
    if flags.unocss {
        features.push("unocss".into());
    }
    if flags.shadcn {
        features.push("shadcn".into());
    }
    if flags.daisyui {
        features.push("daisyui".into());
    }

    // State
    if flags.zustand {
        features.push("zustand".into());
    }
    if flags.redux {
        features.push("redux".into());
    }
    if flags.jotai {
        features.push("jotai".into());
    }
    if flags.recoil {
        features.push("recoil".into());
    }
    if flags.pinia {
        features.push("pinia".into());
    }
    if flags.tanstack_query {
        features.push("tanstack-query".into());
    }

    // Backend
    if flags.express {
        features.push("express".into());
    }
    if flags.fastify {
        features.push("fastify".into());
    }
    if flags.nestjs {
        features.push("nestjs".into());
    }
    if flags.hono {
        features.push("hono".into());
    }
    if flags.koa {
        features.push("koa".into());
    }
    if flags.trpc {
        features.push("trpc".into());
    }

    // Database / ORM
    if flags.prisma {
        features.push("prisma".into());
    }
    if flags.drizzle {
        features.push("drizzle".into());
    }
    if flags.typeorm {
        features.push("typeorm".into());
    }
    if flags.mongoose {
        features.push("mongoose".into());
    }
    if flags.postgres {
        features.push("postgres".into());
    }
    if flags.mysql {
        features.push("mysql".into());
    }
    if flags.sqlite {
        features.push("sqlite".into());
    }
    if flags.mongodb {
        features.push("mongodb".into());
    }

    // Validation
    if flags.zod {
        features.push("zod".into());
    }
    if flags.yup {
        features.push("yup".into());
    }
    if flags.joi {
        features.push("joi".into());
    }
    if flags.valibot {
        features.push("valibot".into());
    }

    // Auth
    if flags.nextauth {
        features.push("nextauth".into());
    }
    if flags.clerk {
        features.push("clerk".into());
    }
    if flags.lucia {
        features.push("lucia".into());
    }
    if flags.jwt {
        features.push("jwt".into());
    }
    if flags.oauth {
        features.push("oauth".into());
    }

    // Testing
    if flags.vitest {
        features.push("vitest".into());
    }
    if flags.jest {
        features.push("jest".into());
    }
    if flags.playwright {
        features.push("playwright".into());
    }
    if flags.cypress {
        features.push("cypress".into());
    }
    if flags.testing_library {
        features.push("testing-library".into());
    }

    // Linting
    if flags.eslint {
        features.push("eslint".into());
    }
    if flags.prettier {
        features.push("prettier".into());
    }
    if flags.biome {
        features.push("biome".into());
    }
    if flags.husky {
        features.push("husky".into());
    }
    if flags.lint_staged {
        features.push("lint-staged".into());
    }
    if flags.commitlint {
        features.push("commitlint".into());
    }

    // Monorepo
    if flags.monorepo {
        features.push("monorepo".into());
    }
    if flags.turborepo {
        features.push("turborepo".into());
    }
    if flags.nx {
        features.push("nx".into());
    }
    if flags.workspaces {
        features.push("workspaces".into());
    }
    if flags.changesets {
        features.push("changesets".into());
    }

    // API
    if flags.rest {
        features.push("rest".into());
    }
    if flags.graphql {
        features.push("graphql".into());
    }
    if flags.trpc_api {
        features.push("trpc-api".into());
    }
    if flags.grpc {
        features.push("grpc".into());
    }

    // Deployment
    if flags.docker || flags.k8s {
        features.push("docker".into());
    }
    if flags.k8s {
        features.push("k8s".into());
    }
    if flags.github_actions {
        features.push("github-actions".into());
    }
    if flags.vercel {
        features.push("vercel".into());
    }
    if flags.railway {
        features.push("railway".into());
    }
    if flags.fly {
        features.push("fly".into());
    }

    // Misc
    if flags.dotenv {
        features.push("dotenv".into());
    }
    if flags.i18n {
        features.push("i18n".into());
    }
    if flags.pwa {
        features.push("pwa".into());
    }
    if flags.storybook {
        features.push("storybook".into());
    }
    if flags.sentry {
        features.push("sentry".into());
    }
    if flags.analytics {
        features.push("analytics".into());
    }

    // Extra
    for feature in &flags.features {
        if !features.contains(feature) {
            features.push(feature.clone());
        }
    }

    features
}

pub(crate) fn parse_framework_request(input: &str) -> FrameworkRequest {
    let (framework, version) = match input.rsplit_once('@') {
        Some((name, selector)) if !name.is_empty() && !selector.is_empty() => {
            (name.to_string(), Some(selector.to_string()))
        }
        _ => (input.to_string(), None),
    };

    let (version, dist_tag) = match version {
        Some(selector) if selector == "latest" => (None, None),
        Some(selector)
            if selector
                .chars()
                .next()
                .is_some_and(|first| first.is_ascii_digit() || matches!(first, '^' | '~')) =>
        {
            (Some(selector), None)
        }
        Some(selector) => (None, Some(selector)),
        None => (None, None),
    };

    FrameworkRequest {
        raw: input.to_string(),
        normalized: normalize_cli_web_framework(&framework),
        version,
        dist_tag,
    }
}

/// Carry the user's selected scaffold ref into native package resolution.
/// Chuyển version/tag user chọn từ scaffold spec sang bước ghi manifest.
fn framework_request_from_spec(
    framework: &str,
    requested_ref: &crate::scaffold::spec::ScaffoldRef,
) -> FrameworkRequest {
    let mut request = parse_framework_request(framework);
    match requested_ref {
        crate::scaffold::spec::ScaffoldRef::Version(version) => {
            request.version = Some(version.clone());
            request.dist_tag = None;
        }
        crate::scaffold::spec::ScaffoldRef::DistTag(tag) if tag != "latest" => {
            request.version = None;
            request.dist_tag = Some(tag.clone());
        }
        crate::scaffold::spec::ScaffoldRef::DistTag(_)
        | crate::scaffold::spec::ScaffoldRef::Default => {
            request.version = None;
            request.dist_tag = None;
        }
        crate::scaffold::spec::ScaffoldRef::Range(_) => {
            // Range refs are rejected by parse_scaffold_spec::validate before this point.
        }
    }
    request
}

fn normalize_cli_web_framework(framework: &str) -> String {
    match framework {
        "ts" | "frontend" | "custom" => "vanilla".to_string(),
        "node" | "backend" => "express".to_string(),
        "fullstack" => "remix".to_string(),
        "go" => "gin".to_string(),
        "java" => "spring-boot".to_string(),
        "php" => "laravel".to_string(),
        "python" => "fastapi".to_string(),
        "rust" => "axum".to_string(),
        "dotnet" => "dotnet-minimal".to_string(),
        "monorepo" => "react-vite".to_string(),
        "react" | "react-app" => "react-vite".to_string(),
        "vue" | "vue-app" => "vue-vite".to_string(),
        "next" | "next-app" => "nextjs".to_string(),
        "svelte" => "sveltekit".to_string(),
        "solid" | "solid-app" => "solidjs".to_string(),
        "angular-app" => "angular".to_string(),
        "qwik-app" => "qwik".to_string(),
        // Fullstack combos with non-Node backends: frontend half resolves to
        // the shared frontend leaf (react-vite / vue-vite / sveltekit).
        "react-axum" | "react-actix-web" | "react-gin" | "react-echo" | "react-fiber"
        | "react-fastapi" | "react-django" | "react-flask" | "react-quarkus" | "react-symfony" => {
            "react-vite".to_string()
        }
        "vue-axum" | "vue-actix-web" | "vue-gin" | "vue-echo" | "vue-fiber" | "vue-fastapi"
        | "vue-django" | "vue-flask" | "vue-quarkus" | "vue-symfony" => "vue-vite".to_string(),
        "svelte-axum" | "svelte-gin" | "svelte-fastapi" | "svelte-quarkus" => {
            "sveltekit".to_string()
        }
        other => other.to_string(),
    }
}

/// Turn broad capability aliases into deterministic scaffold defaults.
/// Biến alias tổng quát thành lựa chọn scaffold có mặc định ổn định.
fn apply_framework_alias_defaults(framework: &str, flags: &mut ScaffoldFlags) {
    match framework {
        "ts" => flags.ts = true,
        "monorepo" => {
            flags.monorepo = true;
            let has_frontend = [
                flags.react,
                flags.next,
                flags.vue,
                flags.nuxt,
                flags.svelte,
                flags.sveltekit,
                flags.solid,
                flags.astro,
                flags.remix,
            ]
            .into_iter()
            .any(|enabled| enabled);
            if !has_frontend {
                flags.react = true;
            }
            if detect_backend_framework(flags).is_none() {
                flags.express = true;
            }
        }
        _ => {}
    }
}

struct WebFrameworkSeed {
    name: &'static str,
    /// Packages that use the user's requested version
    primary: &'static [&'static str],
    /// Packages resolved independently from the registry latest path
    supplemental: &'static [&'static str],
    toolchain: &'static [WebToolchainPackage],
}

struct WebToolchainPackage {
    section: &'static str,
    package: &'static str,
    typescript_only: bool,
    /// If set, use this version instead of fetching from npm registry.
    version: Option<&'static str>,
}

const FRAMEWORK_SEEDS: &[WebFrameworkSeed] = &[
    WebFrameworkSeed {
        name: "vanilla",
        primary: &[],
        supplemental: &[],
        toolchain: &[
            WebToolchainPackage {
                section: "devDependencies",
                package: "vite",
                typescript_only: false,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "typescript",
                typescript_only: true,
                version: None,
            },
        ],
    },
    WebFrameworkSeed {
        name: "solidjs",
        primary: &["solid-js"],
        supplemental: &[],
        toolchain: &[
            WebToolchainPackage {
                section: "devDependencies",
                package: "vite",
                typescript_only: false,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "vite-plugin-solid",
                typescript_only: false,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "typescript",
                typescript_only: true,
                version: None,
            },
        ],
    },
    WebFrameworkSeed {
        name: "sveltekit",
        primary: &["@sveltejs/kit"],
        supplemental: &[],
        toolchain: &[
            WebToolchainPackage {
                section: "devDependencies",
                package: "vite",
                typescript_only: false,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@sveltejs/vite-plugin-svelte",
                typescript_only: false,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@sveltejs/adapter-auto",
                typescript_only: false,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "svelte",
                typescript_only: false,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "typescript",
                typescript_only: true,
                version: Some("^6.0.3"),
            },
        ],
    },
    WebFrameworkSeed {
        name: "react-vite",
        primary: &["react", "react-dom"],
        supplemental: &[],
        toolchain: &[
            WebToolchainPackage {
                section: "devDependencies",
                package: "vite",
                typescript_only: false,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@vitejs/plugin-react",
                typescript_only: false,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "typescript",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@types/react",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@types/react-dom",
                typescript_only: true,
                version: None,
            },
        ],
    },
    WebFrameworkSeed {
        name: "nextjs",
        primary: &["next"],
        supplemental: &["react", "react-dom"],
        toolchain: &[
            WebToolchainPackage {
                section: "devDependencies",
                package: "typescript",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@types/node",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@types/react",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@types/react-dom",
                typescript_only: true,
                version: None,
            },
        ],
    },
    WebFrameworkSeed {
        name: "fastify",
        primary: &["fastify"],
        supplemental: &[],
        toolchain: &[
            WebToolchainPackage {
                section: "devDependencies",
                package: "typescript",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@types/node",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "tsx",
                typescript_only: true,
                version: None,
            },
        ],
    },
    WebFrameworkSeed {
        name: "vue-vite",
        primary: &["vue"],
        supplemental: &[],
        toolchain: &[
            WebToolchainPackage {
                section: "devDependencies",
                package: "vite",
                typescript_only: false,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@vitejs/plugin-vue",
                typescript_only: false,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "typescript",
                typescript_only: true,
                version: None,
            },
        ],
    },
    WebFrameworkSeed {
        name: "nuxt",
        primary: &["nuxt"],
        supplemental: &["vue"],
        toolchain: &[
            WebToolchainPackage {
                section: "devDependencies",
                package: "unplugin",
                typescript_only: false,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "typescript",
                typescript_only: true,
                version: None,
            },
        ],
    },
    WebFrameworkSeed {
        name: "angular",
        primary: &[
            "@angular/core",
            "@angular/compiler",
            "@angular/common",
            "@angular/platform-browser",
            "@angular/platform-browser-dynamic",
            "@angular/router",
        ],
        supplemental: &["rxjs", "zone.js", "tslib"],
        toolchain: &[
            WebToolchainPackage {
                section: "devDependencies",
                package: "@angular/cli",
                typescript_only: false,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@angular/compiler-cli",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@angular-devkit/build-angular",
                typescript_only: false,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "typescript",
                typescript_only: true,
                // Angular 22.2 supports TypeScript >=6.0.0 <6.1.0; the
                // global latest major (TypeScript 7) breaks compiler-cli.
                // (Angular 22.2 cần TypeScript >=6.0.0 <6.1.0; major mới
                // nhất TypeScript 7 làm lỗi compiler-cli.)
                version: Some("~6.0.3"),
            },
        ],
    },
    WebFrameworkSeed {
        name: "qwik",
        primary: &["@builder.io/qwik"],
        supplemental: &[],
        toolchain: &[
            WebToolchainPackage {
                section: "devDependencies",
                package: "@builder.io/qwik-city",
                typescript_only: false,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "vite",
                typescript_only: false,
                version: Some("^7.3.6"),
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "typescript",
                typescript_only: true,
                version: None,
            },
        ],
    },
    WebFrameworkSeed {
        name: "astro",
        primary: &["astro"],
        supplemental: &[],
        toolchain: &[WebToolchainPackage {
            section: "devDependencies",
            package: "typescript",
            typescript_only: true,
            version: None,
        }],
    },
    WebFrameworkSeed {
        name: "express",
        primary: &["express"],
        supplemental: &[],
        toolchain: &[
            WebToolchainPackage {
                section: "devDependencies",
                package: "typescript",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@types/express",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@types/node",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "tsx",
                typescript_only: true,
                version: None,
            },
        ],
    },
    WebFrameworkSeed {
        name: "hono",
        primary: &["hono"],
        supplemental: &["@hono/node-server"],
        toolchain: &[
            WebToolchainPackage {
                section: "devDependencies",
                package: "typescript",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@types/node",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "tsx",
                typescript_only: true,
                version: None,
            },
        ],
    },
    WebFrameworkSeed {
        name: "nestjs",
        primary: &["@nestjs/core", "@nestjs/common", "@nestjs/platform-express"],
        supplemental: &["reflect-metadata", "rxjs"],
        toolchain: &[
            WebToolchainPackage {
                section: "devDependencies",
                package: "typescript",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@types/node",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "tsx",
                typescript_only: true,
                version: None,
            },
        ],
    },
    WebFrameworkSeed {
        name: "trpc",
        primary: &["@trpc/server"],
        supplemental: &["express", "zod"],
        toolchain: &[
            WebToolchainPackage {
                section: "devDependencies",
                package: "typescript",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@types/express",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "@types/node",
                typescript_only: true,
                version: None,
            },
            WebToolchainPackage {
                section: "devDependencies",
                package: "tsx",
                typescript_only: true,
                version: None,
            },
        ],
    },
];

struct WebFeaturePackage {
    feature: &'static str,
    section: &'static str,
    package: &'static str,
}

const FEATURE_PACKAGES: &[WebFeaturePackage] = &[
    WebFeaturePackage {
        feature: "prisma",
        section: "dependencies",
        package: "@prisma/client",
    },
    WebFeaturePackage {
        feature: "prisma",
        section: "devDependencies",
        package: "prisma",
    },
    WebFeaturePackage {
        feature: "vitest",
        section: "devDependencies",
        package: "vitest",
    },
    WebFeaturePackage {
        feature: "eslint",
        section: "devDependencies",
        package: "eslint",
    },
    WebFeaturePackage {
        feature: "eslint",
        section: "devDependencies",
        package: "eslint-config-next",
    },
    WebFeaturePackage {
        feature: "prettier",
        section: "devDependencies",
        package: "prettier",
    },
    WebFeaturePackage {
        feature: "tailwindcss",
        section: "devDependencies",
        package: "@tailwindcss/postcss",
    },
    WebFeaturePackage {
        feature: "tailwindcss",
        section: "devDependencies",
        package: "@tailwindcss/vite",
    },
    WebFeaturePackage {
        feature: "postgres",
        section: "dependencies",
        package: "pg",
    },
    WebFeaturePackage {
        feature: "zustand",
        section: "dependencies",
        package: "zustand",
    },
    WebFeaturePackage {
        feature: "tanstack-query",
        section: "dependencies",
        package: "@tanstack/react-query",
    },
    WebFeaturePackage {
        feature: "zod",
        section: "dependencies",
        package: "zod",
    },
    WebFeaturePackage {
        feature: "shadcn",
        section: "dependencies",
        package: "clsx",
    },
    WebFeaturePackage {
        feature: "shadcn",
        section: "dependencies",
        package: "tailwind-merge",
    },
    WebFeaturePackage {
        feature: "shadcn",
        section: "dependencies",
        package: "class-variance-authority",
    },
    WebFeaturePackage {
        feature: "nextauth",
        section: "dependencies",
        package: "next-auth",
    },
    WebFeaturePackage {
        feature: "playwright",
        section: "devDependencies",
        package: "@playwright/test",
    },
    WebFeaturePackage {
        feature: "husky",
        section: "devDependencies",
        package: "husky",
    },
    WebFeaturePackage {
        feature: "lint-staged",
        section: "devDependencies",
        package: "lint-staged",
    },
    WebFeaturePackage {
        feature: "biome",
        section: "devDependencies",
        package: "@biomejs/biome",
    },
    WebFeaturePackage {
        feature: "sass",
        section: "devDependencies",
        package: "sass",
    },
    WebFeaturePackage {
        feature: "unocss",
        section: "devDependencies",
        package: "unocss",
    },
    WebFeaturePackage {
        feature: "daisyui",
        section: "devDependencies",
        package: "daisyui",
    },
    WebFeaturePackage {
        feature: "redux",
        section: "dependencies",
        package: "@reduxjs/toolkit",
    },
    WebFeaturePackage {
        feature: "redux",
        section: "dependencies",
        package: "react-redux",
    },
    WebFeaturePackage {
        feature: "jest",
        section: "devDependencies",
        package: "jest",
    },
    WebFeaturePackage {
        feature: "jest",
        section: "devDependencies",
        package: "@testing-library/react",
    },
    WebFeaturePackage {
        feature: "jest",
        section: "devDependencies",
        package: "@testing-library/jest-dom",
    },
    WebFeaturePackage {
        feature: "jest",
        section: "devDependencies",
        package: "jest-environment-jsdom",
    },
    WebFeaturePackage {
        feature: "cypress",
        section: "devDependencies",
        package: "cypress",
    },
    WebFeaturePackage {
        feature: "drizzle",
        section: "dependencies",
        package: "drizzle-orm",
    },
    WebFeaturePackage {
        feature: "drizzle",
        section: "devDependencies",
        package: "drizzle-kit",
    },
    WebFeaturePackage {
        feature: "biome",
        section: "devDependencies",
        package: "@biomejs/biome",
    },
    WebFeaturePackage {
        feature: "sass",
        section: "devDependencies",
        package: "sass",
    },
    WebFeaturePackage {
        feature: "unocss",
        section: "devDependencies",
        package: "unocss",
    },
    WebFeaturePackage {
        feature: "daisyui",
        section: "devDependencies",
        package: "daisyui",
    },
    WebFeaturePackage {
        feature: "redux",
        section: "dependencies",
        package: "@reduxjs/toolkit",
    },
    WebFeaturePackage {
        feature: "redux",
        section: "dependencies",
        package: "react-redux",
    },
    WebFeaturePackage {
        feature: "jest",
        section: "devDependencies",
        package: "jest",
    },
    WebFeaturePackage {
        feature: "jest",
        section: "devDependencies",
        package: "@testing-library/react",
    },
    WebFeaturePackage {
        feature: "jest",
        section: "devDependencies",
        package: "@testing-library/jest-dom",
    },
    WebFeaturePackage {
        feature: "jest",
        section: "devDependencies",
        package: "jest-environment-jsdom",
    },
    WebFeaturePackage {
        feature: "cypress",
        section: "devDependencies",
        package: "cypress",
    },
    WebFeaturePackage {
        feature: "drizzle",
        section: "dependencies",
        package: "drizzle-orm",
    },
    WebFeaturePackage {
        feature: "drizzle",
        section: "devDependencies",
        package: "drizzle-kit",
    },
    WebFeaturePackage {
        feature: "clerk",
        section: "dependencies",
        package: "@clerk/nextjs",
    },
    WebFeaturePackage {
        feature: "sass",
        section: "devDependencies",
        package: "sass",
    },
    WebFeaturePackage {
        feature: "styled-components",
        section: "dependencies",
        package: "styled-components",
    },
    WebFeaturePackage {
        feature: "styled-components",
        section: "devDependencies",
        package: "@types/styled-components",
    },
    WebFeaturePackage {
        feature: "commitlint",
        section: "devDependencies",
        package: "@commitlint/cli",
    },
    WebFeaturePackage {
        feature: "commitlint",
        section: "devDependencies",
        package: "@commitlint/config-conventional",
    },
    WebFeaturePackage {
        feature: "rest",
        section: "dependencies",
        package: "express",
    },
    WebFeaturePackage {
        feature: "graphql",
        section: "dependencies",
        package: "@apollo/server",
    },
    WebFeaturePackage {
        feature: "graphql",
        section: "dependencies",
        package: "graphql",
    },
    WebFeaturePackage {
        feature: "graphql",
        section: "dependencies",
        package: "@as-integrations/next",
    },
    WebFeaturePackage {
        feature: "trpc",
        section: "dependencies",
        package: "@trpc/server",
    },
    WebFeaturePackage {
        feature: "storybook",
        section: "devDependencies",
        package: "@storybook/react",
    },
    WebFeaturePackage {
        feature: "storybook",
        section: "devDependencies",
        package: "@storybook/addon-essentials",
    },
    WebFeaturePackage {
        feature: "i18n",
        section: "devDependencies",
        package: "i18next",
    },
    WebFeaturePackage {
        feature: "i18n",
        section: "devDependencies",
        package: "react-i18next",
    },
    WebFeaturePackage {
        feature: "i18n",
        section: "devDependencies",
        package: "i18next-browser-languagedetector",
    },
    WebFeaturePackage {
        feature: "i18n",
        section: "devDependencies",
        package: "i18next-http-backend",
    },
    WebFeaturePackage {
        feature: "fastify",
        section: "dependencies",
        package: "fastify",
    },
    WebFeaturePackage {
        feature: "nestjs",
        section: "dependencies",
        package: "@nestjs/core",
    },
    WebFeaturePackage {
        feature: "nestjs",
        section: "dependencies",
        package: "@nestjs/common",
    },
    WebFeaturePackage {
        feature: "nestjs",
        section: "dependencies",
        package: "@nestjs/platform-express",
    },
    WebFeaturePackage {
        feature: "hono",
        section: "dependencies",
        package: "hono",
    },
    WebFeaturePackage {
        feature: "koa",
        section: "dependencies",
        package: "koa",
    },
    WebFeaturePackage {
        feature: "koa",
        section: "dependencies",
        package: "@koa/router",
    },
    WebFeaturePackage {
        feature: "typeorm",
        section: "dependencies",
        package: "typeorm",
    },
    WebFeaturePackage {
        feature: "typeorm",
        section: "devDependencies",
        package: "@types/typeorm",
    },
    WebFeaturePackage {
        feature: "mongoose",
        section: "dependencies",
        package: "mongoose",
    },
    WebFeaturePackage {
        feature: "mysql",
        section: "dependencies",
        package: "mysql2",
    },
    WebFeaturePackage {
        feature: "sqlite",
        section: "dependencies",
        package: "better-sqlite3",
    },
    WebFeaturePackage {
        feature: "mongodb",
        section: "dependencies",
        package: "mongodb",
    },
    WebFeaturePackage {
        feature: "yup",
        section: "dependencies",
        package: "yup",
    },
    WebFeaturePackage {
        feature: "joi",
        section: "dependencies",
        package: "joi",
    },
    WebFeaturePackage {
        feature: "valibot",
        section: "dependencies",
        package: "valibot",
    },
    WebFeaturePackage {
        feature: "lucia",
        section: "dependencies",
        package: "lucia",
    },
    WebFeaturePackage {
        feature: "jwt",
        section: "dependencies",
        package: "jsonwebtoken",
    },
    WebFeaturePackage {
        feature: "jwt",
        section: "devDependencies",
        package: "@types/jsonwebtoken",
    },
    WebFeaturePackage {
        feature: "oauth",
        section: "dependencies",
        package: "oauth",
    },
    WebFeaturePackage {
        feature: "testing-library",
        section: "devDependencies",
        package: "@testing-library/react",
    },
    WebFeaturePackage {
        feature: "testing-library",
        section: "devDependencies",
        package: "@testing-library/jest-dom",
    },
    WebFeaturePackage {
        feature: "turborepo",
        section: "devDependencies",
        package: "turbo",
    },
    WebFeaturePackage {
        feature: "nx",
        section: "devDependencies",
        package: "nx",
    },
    WebFeaturePackage {
        feature: "changesets",
        section: "devDependencies",
        package: "@changesets/cli",
    },
    WebFeaturePackage {
        feature: "grpc",
        section: "dependencies",
        package: "@grpc/grpc-js",
    },
    WebFeaturePackage {
        feature: "grpc",
        section: "dependencies",
        package: "@grpc/proto-loader",
    },
    WebFeaturePackage {
        feature: "pwa",
        section: "devDependencies",
        package: "workbox-webpack-plugin",
    },
    WebFeaturePackage {
        feature: "sentry",
        section: "dependencies",
        package: "@sentry/node",
    },
    WebFeaturePackage {
        feature: "sentry",
        section: "dependencies",
        package: "@sentry/react",
    },
    WebFeaturePackage {
        feature: "analytics",
        section: "dependencies",
        package: "@vercel/analytics",
    },
];

fn fullstack_backend_framework(framework: &str) -> Option<&'static str> {
    // Explicit combos (legacy + Rust/Go/Python/Java/PHP backends).
    // Backend name is the template folder under templates/web/backend/<lang>/.
    match framework {
        "react-fastify" => Some("fastify"),
        "react-spring" => Some("spring-boot"),
        "vue-laravel" => Some("laravel"),
        "react-express" => Some("express"),
        "react-hono" => Some("hono"),
        "react-nestjs" => Some("nestjs"),
        "react-trpc" => Some("trpc"),
        "react-axum" => Some("axum"),
        "react-actix-web" => Some("actix-web"),
        "react-gin" => Some("gin"),
        "react-echo" => Some("echo"),
        "react-fiber" => Some("fiber"),
        "react-fastapi" => Some("fastapi"),
        "react-django" => Some("django"),
        "react-flask" => Some("flask"),
        "react-quarkus" => Some("quarkus"),
        "react-symfony" => Some("symfony"),
        "vue-express" => Some("express"),
        "vue-hono" => Some("hono"),
        "vue-nestjs" => Some("nestjs"),
        "vue-axum" => Some("axum"),
        "vue-actix-web" => Some("actix-web"),
        "vue-gin" => Some("gin"),
        "vue-echo" => Some("echo"),
        "vue-fiber" => Some("fiber"),
        "vue-fastapi" => Some("fastapi"),
        "vue-django" => Some("django"),
        "vue-flask" => Some("flask"),
        "vue-quarkus" => Some("quarkus"),
        "vue-symfony" => Some("symfony"),
        "svelte-express" => Some("express"),
        "svelte-hono" => Some("hono"),
        "svelte-axum" => Some("axum"),
        "svelte-fastapi" => Some("fastapi"),
        "svelte-gin" => Some("gin"),
        "svelte-quarkus" => Some("quarkus"),
        "next" | "nextjs" => None, // Next.js has built-in API routes
        "nuxt" | "nuxtjs" => Some("hono"),
        _ => None,
    }
}

fn resolve_seed_name(framework: &str) -> &str {
    match framework {
        "react-fastify" | "react-spring" | "react-express" | "react-hono" | "react-nestjs"
        | "react-trpc" => "react-vite",
        "vue-laravel" | "vue-express" | "vue-hono" | "vue-nestjs" => "vue-vite",
        "svelte-express" | "svelte-hono" => "sveltekit",
        _ => framework,
    }
}

/// Add Tailwind from a UI preset only when its dedicated feature flag is absent.
/// Preset UI chỉ tự thêm Tailwind khi cờ feature riêng chưa bật.
fn ui_preset_needs_implicit_tailwind(flags: &ScaffoldFlags) -> bool {
    !flags.tailwindcss && (flags.shadcn || flags.daisyui)
}

async fn resolve_primary_version(
    request: &FrameworkRequest,
    package: &str,
    age_policy: Option<versions::ScaffoldAgePolicy>,
) -> Result<String> {
    if let Some(tag) = request.dist_tag.as_deref() {
        return versions::resolve_npm_dist_tag_version(package, tag, age_policy).await;
    }
    match request.version.as_deref() {
        Some("latest") | None => versions::fetch_npm_latest_version(package, age_policy).await,
        Some(version) => {
            versions::resolve_seed_dependency_version(package, Some(version), age_policy).await
        }
    }
}

fn ensure_package(root: &mut Map<String, Value>, section: &str, package: &str, version: &str) {
    let entry = root
        .entry(section.to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    if let Value::Object(map) = entry {
        map.insert(package.to_string(), Value::String(version.to_string()));
    }
}

/// Update the existing primary-package section and remove duplicate declarations.
/// Cập nhật section hiện có của primary package và xóa khai báo trùng.
fn ensure_primary_package(root: &mut Map<String, Value>, package: &str, version: &str) {
    const DEPENDENCY_SECTIONS: [&str; 2] = ["dependencies", "devDependencies"];
    let existing_section = DEPENDENCY_SECTIONS.iter().copied().find(|section| {
        root.get(*section)
            .and_then(Value::as_object)
            .is_some_and(|packages| packages.contains_key(package))
    });
    let target_section = existing_section.unwrap_or("dependencies");

    for section in DEPENDENCY_SECTIONS {
        if let Some(packages) = root.get_mut(section).and_then(Value::as_object_mut) {
            if section == target_section {
                packages.insert(package.to_string(), Value::String(version.to_string()));
            } else {
                packages.remove(package);
            }
        }
    }

    if existing_section.is_none() {
        ensure_package(root, "dependencies", package, version);
    }
}

async fn apply_web_manifest_seed(
    package_json_path: &Path,
    request: &FrameworkRequest,
    flags: &ScaffoldFlags,
) -> Result<()> {
    if !package_json_path.exists() {
        return Ok(());
    }
    let mut value: Value = serde_json::from_str(&std::fs::read_to_string(package_json_path)?)?;
    let object = value
        .as_object_mut()
        .ok_or_else(crate::error::package_json_root_object)?;
    let project_root = package_json_path.parent().unwrap_or_else(|| Path::new("."));
    let age_policy = versions::scaffold_age_policy(project_root)?;

    let seed_name = resolve_seed_name(&request.normalized);
    if let Some(seed) = FRAMEWORK_SEEDS.iter().find(|s| s.name == seed_name) {
        // Resolve paired primaries independently; registry publications can be staggered.
        // Giải quyết từng package primary riêng vì registry có thể phát hành lệch thời điểm.
        for &package in seed.primary {
            let primary = resolve_primary_version(request, package, age_policy).await?;
            ensure_primary_package(object, package, &primary);
        }
        for &package in seed.supplemental {
            let version =
                versions::resolve_seed_dependency_version(package, None, age_policy).await?;
            ensure_package(object, "dependencies", package, &version);
        }
        for tool in seed.toolchain {
            if tool.typescript_only && !flags.ts {
                continue;
            }
            let version =
                versions::resolve_seed_dependency_version(tool.package, tool.version, age_policy)
                    .await?;
            ensure_package(object, tool.section, tool.package, &version);
        }
    }

    if ui_preset_needs_implicit_tailwind(flags) {
        let version =
            versions::resolve_seed_dependency_version("tailwindcss", None, age_policy).await?;
        ensure_package(object, "devDependencies", "tailwindcss", &version);
    }

    for feat_pkg in FEATURE_PACKAGES {
        let is_active = match feat_pkg.feature {
            "prisma" => flags.prisma,
            "vitest" => flags.vitest,
            "eslint" => flags.eslint,
            "prettier" => flags.prettier,
            "tailwindcss" => flags.tailwindcss,
            "postgres" => flags.postgres,
            "zustand" => flags.zustand,
            "tanstack-query" => flags.tanstack_query,
            "zod" => flags.zod,
            "shadcn" => flags.shadcn,
            "nextauth" => flags.nextauth,
            "playwright" => flags.playwright,
            "husky" => flags.husky,
            "lint-staged" => flags.lint_staged,
            "biome" => flags.biome,
            "sass" => flags.sass,
            "unocss" => flags.unocss,
            "daisyui" => flags.daisyui,
            "redux" => flags.redux,
            "jest" => flags.jest,
            "cypress" => flags.cypress,
            "drizzle" => flags.drizzle,
            "clerk" => flags.clerk,
            "styled-components" => flags.styled_components,
            "commitlint" => flags.commitlint,
            "rest" => flags.rest,
            "graphql" => flags.graphql,
            "trpc" => flags.trpc,
            "grpc" => flags.grpc,
            "lucia" => flags.lucia,
            "jwt" => flags.jwt,
            "oauth" => flags.oauth,
            "dotenv" => flags.dotenv,
            "i18n" => flags.i18n,
            "pwa" => flags.pwa,
            "storybook" => flags.storybook,
            "sentry" => flags.sentry,
            "analytics" => flags.analytics,
            "railway" => flags.railway,
            "fly" => flags.fly,
            _ => false,
        };
        if is_active {
            let version =
                versions::resolve_seed_dependency_version(feat_pkg.package, None, age_policy)
                    .await?;
            ensure_package(object, feat_pkg.section, feat_pkg.package, &version);
        }
    }

    // P0 finding (2026-09-12): `mgc test` fails honestly ("No test
    // runner detected") on scaffolds that ship no test script — every
    // fullstack/vite template that carries TypeScript must ship a
    // `test` script (tsc --noEmit typecheck is the honest baseline
    // smoke; vitest/jest flags override it via ensure_script below).
    // This is the scaffold's OWN test — no CI printf-ing tests into
    // user projects at run time.
    // P0 finding (2026-09-12): `mgc test` fail trung thực ("No test
    // runner detected") trên template không có script test — mọi
    // template vite có TypeScript phải TỰ mang script `test` (tsc
    // --noEmit là smoke baseline trung thực; flag vitest/jest ghi đè
    // qua ensure_script ở dưới). Đây là test CỦA scaffold — CI không
    // bịa test vào project user lúc chạy.
    if flags.ts
        && matches!(
            seed_name,
            "react-vite" | "vue-vite" | "solid" | "svelte" | "qwik"
        )
    {
        ensure_script(object, "test", "tsc --noEmit");
    }

    std::fs::write(package_json_path, serde_json::to_string_pretty(&value)?)?;
    Ok(())
}

/// Insert a scripts entry only when the scaffold did not define it —
/// user/template choices win over the default baseline.
/// Chèn mục scripts chỉ khi scaffold chưa định nghĩa — lựa chọn của
/// user/template thắng baseline mặc định.
fn ensure_script(root: &mut Map<String, Value>, name: &str, command: &str) {
    let scripts = root
        .entry("scripts".to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    if let Value::Object(map) = scripts {
        map.entry(name.to_string())
            .or_insert_with(|| Value::String(command.to_string()));
    }
}

pub(crate) async fn enrich_web_project_manifest(
    project_dir: &Path,
    frontend: &FrameworkRequest,
    backend: Option<&FrameworkRequest>,
    flags: &ScaffoldFlags,
) -> Result<()> {
    if flags.monorepo {
        apply_web_manifest_seed(
            &project_dir
                .join("apps")
                .join("frontend")
                .join("package.json"),
            frontend,
            flags,
        )
        .await?;
        if let Some(backend) = backend {
            apply_web_manifest_seed(
                &project_dir
                    .join("apps")
                    .join("backend")
                    .join("package.json"),
                backend,
                flags,
            )
            .await?;
        }
        return Ok(());
    }
    if let Some(backend) = backend {
        apply_web_manifest_seed(&project_dir.join("package.json"), frontend, flags).await?;
        apply_web_manifest_seed(
            &project_dir.join("server").join("package.json"),
            backend,
            flags,
        )
        .await
    } else {
        apply_web_manifest_seed(&project_dir.join("package.json"), frontend, flags).await
    }
}

#[cfg(test)]
#[path = "test/web.rs"]
mod tests;
