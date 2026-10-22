//! Public project-configuration facade.
//! Facade công khai cho cấu hình project.

mod project;

pub use project::{
    AiConfig, AppConfig, CicdConfig, CloudConfig, CompatConfig, DEFAULT_MIN_RELEASE_AGE_HOURS,
    DedupeConfig, GameConfig, IotConfig, LibConfig, LockConfig, ProjectConfig,
    ProjectExecutionConfig, ProjectIdentitySnapshot, ScriptGrammarConfig, ScriptVerdict,
    ScriptsPolicy, ScriptsPolicyError, SecurityConfig, SourceConfig, TrustConfig, decide_scripts,
    default_script_grammar, load_scripts_table, load_scripts_table_typed, load_security_config,
    lock_schema_is_v4, policy_lists, project_identity_matches_snapshot, read_regular_project_text,
    restore_project_identity, snapshot_project_identity, unsigned_artifact_allowed,
};
