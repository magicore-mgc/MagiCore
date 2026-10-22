//! Project configuration implementation and child-owner wiring.
//! Wiring implementation cấu hình project và các owner con.

#[path = "core_detection/mod.rs"]
mod core_detection;
#[path = "core_identity/mod.rs"]
mod core_identity;
#[path = "core_marker/mod.rs"]
mod core_marker;
#[path = "file_access/mod.rs"]
mod file_access;
#[path = "project_root/mod.rs"]
mod project_root;
#[path = "security/mod.rs"]
mod security;

pub use core_identity::{
    ProjectIdentitySnapshot, project_identity_matches_snapshot, restore_project_identity,
    snapshot_project_identity,
};
use file_access::atomic_write_project_config;
pub use file_access::read_regular_project_text;
pub use security::{
    SecurityConfig, load_scripts_table, load_security_config, unsigned_artifact_allowed,
};

#[path = "execution/mod.rs"]
mod execution;
pub use execution::ProjectExecutionConfig;
use execution::default_execution_for;
#[path = "core_settings/mod.rs"]
mod core_settings;
pub use core_settings::{
    AiConfig, AppConfig, CicdConfig, CloudConfig, GameConfig, IotConfig, LibConfig,
};
#[path = "dedupe/mod.rs"]
mod dedupe;
pub use dedupe::DedupeConfig;
#[path = "scripts/mod.rs"]
mod scripts;
pub use scripts::{
    ScriptVerdict, ScriptsPolicy, ScriptsPolicyError, decide_scripts,
    load_scripts_table as load_scripts_table_typed, policy_lists,
};
#[path = "lock/mod.rs"]
mod lock;
pub use lock::LockConfig;
#[path = "trust/mod.rs"]
mod trust;
pub use trust::TrustConfig;
#[path = "compat/mod.rs"]
mod compat;
pub use compat::CompatConfig;
#[path = "registry_sources/mod.rs"]
mod registry_sources;
pub use registry_sources::SourceConfig;
#[path = "scaffold_defaults/mod.rs"]
mod scaffold_defaults;
#[path = "script_grammar/mod.rs"]
mod script_grammar;
pub use script_grammar::{ScriptGrammarConfig, default_script_grammar};

#[path = "project_config/mod.rs"]
mod project_config;
pub use project_config::ProjectConfig;
#[path = "lock_schema/mod.rs"]
mod lock_schema;
#[path = "project_config_io/mod.rs"]
mod project_config_io;
pub use lock_schema::lock_schema_is_v4;

/// Default minimum release age for supply-chain quarantine, in hours.
/// Tuổi phát hành mặc định để cách ly chuỗi cung ứng, tính bằng giờ.
pub const DEFAULT_MIN_RELEASE_AGE_HOURS: u64 = 24;
