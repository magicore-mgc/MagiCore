//! Preflight and resource setup for one Web install.
//! Preflight và chuẩn bị resource cho một lượt Web install.

use crate::cache::SharedWebCache;
use crate::install::TreeBackup;
use crate::install::recover_interrupted_install;
use crate::install::run_install::state::InstallState;
use crate::native::npm_registry::NpmRegistry;
use mgc_store::{ContentStore, Database, Layout, PackageCache};

/// Whether the calling CLI already holds this project's install lock.
/// CLI gọi vào đã giữ lock install của project này chưa.
///
/// Set by the CLI guard, which is the single serialization point every core
/// uses. It is a marker, not a bypass: the guard is an OS file lock that is
/// already held, and taking it twice in one process deadlocks.
/// (Do guard của CLI đặt, là điểm tuần tự hoá duy nhất mọi core dùng. Đây là
/// dấu hiệu chứ không phải bypass: guard là OS file lock đã giữ sẵn, lấy hai
/// lần trong cùng process sẽ deadlock.)
fn caller_holds_project_lock() -> bool {
    std::env::var("MGC_PROJECT_LOCK_HELD").is_ok_and(|value| value == "1")
}
use mgc_types::adapter::{InstallOptions, InstallSummary, ResolvedGraph};
use mgc_types::{MgError, MgResult};
use rayon::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::time::Instant;

pub(in crate::install::run_install) fn initialize<'a>(
    registry: &'a NpmRegistry,
    store_override: Option<&'a ContentStore>,
    shared_cache: Option<SharedWebCache>,
    prefetch_handle: Option<tokio::task::JoinHandle<MgResult<u64>>>,
    graph: &'a ResolvedGraph,
    project_root: &'a Path,
    opts: InstallOptions,
) -> MgResult<InstallState<'a>> {
    let registry_url = registry.registry_url();
    // Patch specs are integrity-managed but are not applied by the production
    // materializer. Refuse before creating stores or touching node_modules.
    // Từ chối patch trước khi tạo store hoặc đụng node_modules.
    if let Some(project) =
        mgc_config::project::ProjectConfig::load(project_root).map_err(|error| {
            MgError::Other(format!(
                "cannot inspect project patch configuration: {error}"
            ))
        })?
        && !project.patches.is_empty()
    {
        return Err(MgError::Unsupported {
            core: "web",
            capability: "patched install",
            guidance: "the configured patch is not yet applied by MagiCore's production install pipeline; remove the patch entry or use a build that supports patch materialization".to_string(),
        });
    }

    let node_modules = project_root.join("node_modules");
    TreeBackup::preflight_target(&node_modules)?;

    let start = Instant::now();
    let profile = crate::profile::InstallProfile::from_env();
    let layout = Layout::new(crate::lockfile::project_cache_dir(project_root));
    std::fs::create_dir_all(layout.root()).map_err(|error| {
        MgError::Other(format!(
            "failed to create layout root '{}': {error}",
            layout.root().display()
        ))
    })?;
    std::fs::create_dir_all(layout.temp_dir()).map_err(|error| {
        MgError::Other(format!(
            "failed to create temp dir '{}': {error}",
            layout.temp_dir().display()
        ))
    })?;

    // SEC-01: the tarball cache is keyed by (registry origin, name, version).
    // Entries written from one registry are never served to another.
    // SEC-01: cache tarball khoá theo (origin registry, name, version).
    // Entry ghi từ registry này không bao giờ được phục vụ cho registry khác.
    let cache = PackageCache::for_registry(layout.cache_dir(), registry_url)
        .map_err(|error| MgError::Store(error.to_string()))?;
    eprintln!("[magicore:debug] install:cache_open=ok");
    let database =
        Some(Database::open(&layout.db_path()).map_err(|error| MgError::Store(error.to_string()))?);
    eprintln!("[magicore:debug] install:db_open=ok");
    let default_store =
        ContentStore::new(layout.cas_dir()).map_err(|error| MgError::Store(error.to_string()))?;
    eprintln!("[magicore:debug] install:cas_open=ok");
    std::fs::create_dir_all(&node_modules).map_err(|error| {
        MgError::Other(format!(
            "failed to create '{}': {error}",
            node_modules.display()
        ))
    })?;
    let summary = InstallSummary::default();

    // The project lock serializes node_modules and lockfile mutation. Its OS
    // file lock is released automatically when a crashed process exits.
    // Khóa file theo project tuần tự hóa mutation và tự nhả khi process chết.
    // SEC-07: key on the project root (the CLI guard's key) inside the shared
    // store lock root, so every core contends on one file per project.
    // (SEC-07: khoá theo project root (cùng key với CLI guard) trong root khoá
    // store dùng chung, để mọi core tranh một file cho mỗi project.)
    // SEC-07: one lock key and one lock root for every core — the same values
    // the CLI guard uses (`mgc_store::default_store_root()/locks`, keyed on the
    // canonical project path). An install that is already wrapped by the guard
    // (that caller lives in another crate) would deadlock, so the CLI passes a
    // pre-acquired lock through `MGC_PROJECT_LOCK_HELD`; without it the adapter
    // takes the lock itself and two concurrent `mgc install-web` runs still
    // serialize.
    // (SEC-07: một key và một root khoá cho mọi core — đúng giá trị guard CLI
    // dùng (`mgc_store::default_store_root()/locks`, khoá theo path canonical).
    // Lần install đã được guard bao sẵn (caller ở crate khác) sẽ deadlock, nên
    // CLI truyền qua `MGC_PROJECT_LOCK_HELD`; không có biến đó adapter tự lấy
    // lock và hai `mgc install-web` song song vẫn tuần tự hoá.)
    let recovered = recover_interrupted_install(project_root, &node_modules, &layout.temp_dir())?;
    if recovered.restored_backup || recovered.removed_staging > 0 {
        eprintln!(
            "[magicore] recovered interrupted install (backup restored: {}, staging removed: {}, snaps swept: {})",
            recovered.restored_backup, recovered.removed_staging, recovered.removed_snaps
        );
    }

    // Every early return drops this guard and aborts only this staging token.
    // Mọi return sớm đều drop guard và abort đúng staging token này.
    let (cas_generation, cas_generation_guard) = match database.as_ref() {
        Some(database) => {
            let project_key = layout.root().to_string_lossy().to_string();
            let guard = database
                .begin_cas_generation_guarded(&layout.db_path(), &project_key)
                .map_err(|error| MgError::Store(error.to_string()))?;
            let token = guard.generation();
            (token, Some(guard))
        }
        None => (0, None),
    };
    mgc_store::failpoint::hit("after-generation-begin");

    let thread_id_hash = {
        let thread_id = std::thread::current().id();
        let mut hasher = DefaultHasher::new();
        thread_id.hash(&mut hasher);
        hasher.finish()
    };
    let staging_root = if opts.legacy_flat {
        let root = layout.temp_dir().join(format!(
            "install-stage-{}-{}-{:x}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos(),
            thread_id_hash
        ));
        std::fs::create_dir_all(root.join("node_modules")).map_err(|error| {
            MgError::Other(format!(
                "failed to create staging node_modules '{}': {error}",
                root.join("node_modules").display()
            ))
        })?;
        Some(root)
    } else {
        None
    };
    let root_packages = crate::install::materialize::select_root_packages(graph);
    let all_root_matched = !root_packages.is_empty()
        && root_packages.par_iter().all(|package| {
            crate::lockfile::installed_package_matches(
                &node_modules.join(package.id.name().as_str()),
                &package.id,
            )
        });

    Ok(InstallState {
        registry,
        registry_url,
        store_override,
        shared_cache,
        prefetch_handle,
        graph,
        project_root,
        opts,
        start,
        profile,
        layout,
        cache,
        database,
        default_store,
        node_modules,
        summary,
        _install_lock: if caller_holds_project_lock() {
            None
        } else {
            let lock_root = mgc_store::default_store_root().join("locks");
            Some(
                mgc_store::ProjectInstallLock::acquire_at(
                    &lock_root,
                    &project_root.to_string_lossy(),
                )
                .map_err(|error| MgError::Store(error.to_string()))?,
            )
        },
        cas_generation,
        cas_generation_guard,
        staging_root,
        root_packages,
        all_root_matched,
        root_package_versions: Default::default(),
        package_map: Default::default(),
        packages_with_scripts: Vec::new(),
        already_materialized: Default::default(),
        already_in_virtual_store: Default::default(),
        shared_package_cache_for_install: None,
        use_shared_primary: false,
        fetch_graph: ResolvedGraph::empty(),
        prior_lock: None,
        tree_backup: None,
    })
}
