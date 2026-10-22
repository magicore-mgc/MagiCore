//! Explicit compatibility escape hatches for project dependency resolution.
//! Escape hatch tương thích tường minh cho phân giải dependency của project.

use serde::{Deserialize, Serialize};

/// Compatibility opt-ins — `mgc.toml [compat]`, default-deny outside listed options.
/// Opt-in tương thích `[compat]`; mặc định chặn các lựa chọn ngoài danh sách.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompatConfig {
    /// Allow git-dependency resolution — cho phép phân giải dependency Git.
    #[serde(default)]
    pub allow_git_deps: bool,
    /// Host allowlist for git dependencies — allowlist host cho dependency Git.
    #[serde(default)]
    pub git_hosts: Vec<String>,
}
