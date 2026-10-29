//! Explicit state carried through the Web install phases.
//! Trạng thái tường minh được truyền qua các phase Web install.

use crate::cache::SharedWebCache;
use crate::install::TreeBackup;
use crate::install::script_policy::LifecyclePackage;
use crate::native::npm_registry::NpmRegistry;
use mgc_store::{ContentStore, Database, Layout, PackageCache};
use mgc_types::adapter::{InstallOptions, InstallSummary, ResolvedGraph, ResolvedPackage};
use mgc_types::{MgResult, PackageId};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tokio::task::JoinHandle;

/// All resources and phase outputs for one serialized project install.
/// Resource và kết quả phase của một lượt install project đã khóa.
pub(in crate::install::run_install) struct InstallState<'a> {
    pub(in crate::install::run_install) registry: &'a NpmRegistry,
    pub(in crate::install::run_install) registry_url: &'a str,
    pub(in crate::install::run_install) store_override: Option<&'a ContentStore>,
    pub(in crate::install::run_install) shared_cache: Option<SharedWebCache>,
    pub(in crate::install::run_install) prefetch_handle: Option<JoinHandle<MgResult<u64>>>,
    pub(in crate::install::run_install) graph: &'a ResolvedGraph,
    pub(in crate::install::run_install) project_root: &'a Path,
    pub(in crate::install::run_install) opts: InstallOptions,
    pub(in crate::install::run_install) start: Instant,
    pub(in crate::install::run_install) profile: crate::profile::InstallProfile,
    pub(in crate::install::run_install) layout: Layout,
    pub(in crate::install::run_install) cache: PackageCache,
    pub(in crate::install::run_install) database: Option<Database>,
    pub(in crate::install::run_install) default_store: ContentStore,
    pub(in crate::install::run_install) node_modules: PathBuf,
    pub(in crate::install::run_install) summary: InstallSummary,
    /// Install lock held for the duration of the run; `None` when the calling
    /// CLI already owns it (SEC-07).
    /// (Lock install giữ suốt lần chạy; `None` khi CLI gọi vào đã giữ —
    /// SEC-07.)
    pub(in crate::install::run_install) _install_lock: Option<mgc_store::ProjectInstallLock>,
    pub(in crate::install::run_install) cas_generation: i64,
    pub(in crate::install::run_install) cas_generation_guard:
        Option<mgc_store::database::CasGenerationGuard>,
    pub(in crate::install::run_install) staging_root: Option<PathBuf>,
    pub(in crate::install::run_install) root_packages: Vec<&'a ResolvedPackage>,
    pub(in crate::install::run_install) all_root_matched: bool,
    pub(in crate::install::run_install) root_package_versions: HashMap<String, PackageId>,
    pub(in crate::install::run_install) package_map: HashMap<PackageId, &'a ResolvedPackage>,
    pub(in crate::install::run_install) packages_with_scripts: Vec<LifecyclePackage>,
    pub(in crate::install::run_install) already_materialized: HashSet<PackageId>,
    pub(in crate::install::run_install) already_in_virtual_store: HashSet<PackageId>,
    pub(in crate::install::run_install) shared_package_cache_for_install: Option<PackageCache>,
    pub(in crate::install::run_install) use_shared_primary: bool,
    pub(in crate::install::run_install) fetch_graph: ResolvedGraph,
    pub(in crate::install::run_install) prior_lock: Option<Vec<u8>>,
    pub(in crate::install::run_install) tree_backup: Option<TreeBackup>,
}

impl InstallState<'_> {
    pub(in crate::install::run_install) fn active_package_cache(&self) -> &PackageCache {
        if self.use_shared_primary {
            self.shared_package_cache_for_install
                .as_ref()
                .expect("shared package cache was checked before selecting it")
        } else {
            &self.cache
        }
    }

    pub(in crate::install::run_install) fn secondary_shared_cache(
        &self,
    ) -> Option<&SharedWebCache> {
        if self.use_shared_primary {
            None
        } else {
            self.shared_cache.as_ref()
        }
    }
}
