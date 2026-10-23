//! Serialized project configuration schema.
//! Schema cấu hình project tuần tự hóa.

use crate::registry::Registry;
use serde::{Deserialize, Serialize};

use super::super::{
    AiConfig, AppConfig, CicdConfig, CloudConfig, CompatConfig, DedupeConfig, GameConfig,
    IotConfig, LibConfig, LockConfig, ProjectExecutionConfig, ScriptGrammarConfig, ScriptsPolicy,
    SecurityConfig, SourceConfig, TrustConfig,
};

/// Project config saved by `mgc init` and read by all commands.
/// Cấu hình do `mgc init` lưu và các lệnh cùng đọc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project name
    /// Tên project.
    pub name: String,
    /// Project version
    /// Phiên bản project.
    #[serde(default = "default_version")]
    pub version: String,
    /// Ecosystem / core type (web, game, ai, cloud, iot, app, lib)
    /// Hệ sinh thái hoặc core MagiCore.
    pub ecosystem: String,
    /// Mode (frontend, backend, fullstack, monorepo) — web core only
    /// Chế độ dự án Web; core khác để trống.
    #[serde(default)]
    pub mode: String,
    /// Frameworks used (e.g. ["react-vite"] or ["node", "express"])
    /// Danh sách framework được cấu hình cho project.
    #[serde(default)]
    pub frameworks: Vec<String>,
    /// Template path used during scaffold
    /// Đường dẫn template đã dùng khi scaffold.
    #[serde(default)]
    pub template: String,
    /// Features selected (e.g. ["ts", "tailwind"])
    /// Những feature được chọn cho project.
    #[serde(default)]
    pub features: Vec<String>,
    /// Execution strategy / runtime lane metadata
    /// Metadata của strategy và runtime lane.
    #[serde(default)]
    pub execution: ProjectExecutionConfig,
    /// Registry configuration (mgc.toml [registries])
    /// Cấu hình registry trong mgc.toml.
    #[serde(default)]
    pub registries: Vec<Registry>,
    /// Package patches (mgc.toml [patches])
    /// Các bản vá package được cấu hình.
    #[serde(default)]
    pub patches: Vec<mgc_types::PatchSpec>,
    /// Dedupe settings (mgc.toml [dedupe]) — opt-in (02 §2.1)
    /// Tùy chọn loại bỏ dependency trùng lặp.
    #[serde(default)]
    pub dedupe: DedupeConfig,
    /// Library core config (mgc.toml [lib]) — language + pip allowlist.
    /// Cấu hình ngôn ngữ và allowlist pip cho core Lib.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lib: Option<LibConfig>,
    /// Game core config (mgc.toml [game]) — engine.
    /// Cấu hình engine của core Game.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game: Option<GameConfig>,
    /// IoT core config (mgc.toml [iot]) — framework and board.
    /// Cấu hình framework và board của core IoT.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iot: Option<IotConfig>,
    /// Cloud core config (mgc.toml [cloud]) — deployment type.
    /// Cấu hình loại triển khai của core Cloud.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<CloudConfig>,
    /// CICD core config (mgc.toml [cicd]) — provider.
    /// Cấu hình provider của core CI/CD.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cicd: Option<CicdConfig>,
    /// App core config (mgc.toml [app]) — language.
    /// Cấu hình ngôn ngữ của core App.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<AppConfig>,
    /// AI core config (mgc.toml [ai]) — framework.
    /// Cấu hình framework của core AI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ai: Option<AiConfig>,
    /// CI config (mgc.toml [ci]) — declared build/test commands for CI scaffolds.
    /// Cấu hình CI trong mgc.toml: lệnh build/test cho scaffold CI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ci: Option<crate::project::CiConfig>,
    /// Security config (mgc.toml [security]) — min_release_age per ecosystem.
    /// Chính sách tuổi phát hành tối thiểu theo hệ sinh thái.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<SecurityConfig>,
    /// Lock config (mgc.toml [lock]) — signature policy and writer lock timeouts.
    /// Chính sách chữ ký và timeout khóa ghi lockfile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock: Option<LockConfig>,
    /// Trust roots (mgc.toml [trust]) — key IDs accepted in `require` mode.
    /// Các key ID tin cậy trong chế độ `require`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust: Option<TrustConfig>,
    /// Compatibility opt-ins (mgc.toml [compat]) — explicit escape hatches.
    /// Tùy chọn tương thích được bật tường minh.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compat: Option<CompatConfig>,
    /// Registry/index sources (mgc.toml [[sources]]) — multi-index source selection.
    /// Chính sách chọn nguồn từ nhiều registry/index.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<SourceConfig>>,
    /// Lifecycle script policy (mgc.toml [scripts]) — committed per-package allow/deny.
    /// Merges with machine-local trust DB; deny wins in both sources.
    /// Policy script đã commit theo package, hợp nhất trust DB máy và luôn ưu tiên deny.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scripts: Option<ScriptsPolicy>,
    /// Native script grammar pin (mgc.toml [script]); unknown versions fail closed.
    /// An absent pin uses the stable default for the ecosystem.
    /// Ghim grammar script native; version lạ bị chặn, còn thiếu dùng default ổn định.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<ScriptGrammarConfig>,
}

fn default_version() -> String {
    "0.1.0".to_string()
}
