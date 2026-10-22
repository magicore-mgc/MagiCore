use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use mgc_lockfile::Lockfile;
use mgc_lockfile::project_lock::ProjectWriteLock;
use mgc_types::adapter::{AddOptions, InstallOptions, PackageAdapter};
use mgc_types::{
    DependencySpec, Ecosystem, Manifest, PackageId, PackageName, ResolvedGraph, ResolvedPackage,
    Version, adapter::PreparedAdd,
};
use mgc_ui::{
    add_multi_bar, create_multi_progress, create_progress_bar, create_spinner, info, style_cmd,
    success,
};
#[cfg(feature = "web")]
use time::{Duration as TimeDuration, OffsetDateTime, format_description::well_known::Rfc3339};

#[path = "mutation_journal/mod.rs"]
mod mutation_journal;
#[cfg(all(test, feature = "lib"))]
use mutation_journal::test_adapter_and_lock;
pub(crate) use mutation_journal::{
    MutationOperation, MutationSnapshot, atomic_write_file_checked,
    ensure_no_pending_remove_journal, finish_mutation_journal, journaled_step, record_post_image,
    refuse_project_link, rollback_mutation, stage_mutation_journal,
};
use mutation_journal::{adopt_legacy_journal, recover_interrupted_remove};
#[cfg(test)]
use mutation_journal::{
    copy_dir_recursive, craft_identity_journal, manifest_canonical_digest, python_fixture,
};

#[path = "install_policy/mod.rs"]
mod install_policy;
#[path = "manifest_delta/mod.rs"]
mod manifest_delta;
use manifest_delta::{AddedPackage, build_delta_manifest, merge_graphs};
#[path = "install_profile/mod.rs"]
mod install_profile;
#[cfg(feature = "ai")]
pub use crate::commands::core::ai::{ai_dev, ai_project_root, require_native_ai_python};
pub use crate::commands::core::web::should_use_legacy_flat_layout;
pub(crate) use install_policy::refresh_locked_graph_for_age_gate;
use install_profile::profile_install_mark;

#[path = "core_identity/mod.rs"]
mod core_identity;

pub(crate) use core_identity::{
    core_project_root, ensure_project_core_identity, find_project_root,
};

#[path = "mutation_gateway/mod.rs"]
mod mutation_gateway;

pub(crate) use mutation_gateway::{begin_dependency_mutation, writer_lock_timeout};

use mutation_gateway::{
    ensure_native_manifest_owner, ensure_native_update_owner, install_command_for_adapter,
};

#[path = "package_add/mod.rs"]
mod package_add;

pub(crate) use package_add::add;

#[path = "package_remove/mod.rs"]
mod package_remove;

pub(crate) use package_remove::remove;

#[path = "package_list/mod.rs"]
mod package_list;

pub(crate) use package_list::list;

#[path = "package_update/mod.rs"]
mod package_update;

pub(crate) use package_update::{native_update, update};

#[path = "install_execution/mod.rs"]
mod install_execution;

pub(crate) use install_execution::{install_with_adapter, prepare_install_execution};

use install_execution::install_with_adapter_locked;

#[path = "audit_policy/mod.rs"]
mod audit_policy;

use audit_policy::enforce_audit_strict_policy;

#[path = "lock_reuse/mod.rs"]
mod lock_reuse;

use lock_reuse::try_install_added_packages_from_lock;

#[path = "locked_graph/mod.rs"]
mod locked_graph;

#[cfg(test)]
pub(crate) use locked_graph::lock_graph_matches_manifest_closure;
pub(crate) use locked_graph::{
    load_locked_graph, load_locked_graph_v4, lock_matches_manifest, lock_owner_core,
    locked_package_ids,
};

#[path = "adapter_factory/mod.rs"]
mod adapter_factory;

#[cfg(any(feature = "game", feature = "clo"))]
pub(crate) use adapter_factory::core_adapter;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
