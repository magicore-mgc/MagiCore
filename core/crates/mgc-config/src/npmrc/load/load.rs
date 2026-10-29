//! User/project npmrc loading and merge / Nạp và gộp npmrc user/project.

use anyhow::Result;
use std::path::Path;

use crate::npmrc::NpmRc;

/// Registry configured by the *user*, ignoring any project file (SEC-03).
/// Registry do *user* cấu hình, bỏ qua file trong project (SEC-03).
///
/// Credential operations (`mgc login`) must never take their destination from
/// a file inside a cloned repository: a planted project `.npmrc` would
/// redirect the password to an attacker's host. Environment wins, then
/// `~/.npmrc`; a project file is deliberately not consulted.
/// Thao tác credential (`mgc login`) không bao giờ lấy đích từ file trong repo
/// clone: `.npmrc` trong project có thể chuyển mật khẩu sang host kẻ tấn công.
/// Env thắng, rồi `~/.npmrc`; file trong project cố ý không đọc.
pub fn user_registry_only() -> Option<String> {
    for key in ["MAGICORE_NPM_REGISTRY", "NPM_CONFIG_REGISTRY"] {
        if let Ok(value) = std::env::var(key) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    let user_npmrc = dirs::home_dir()?.join(".npmrc");
    if !user_npmrc.exists() {
        return None;
    }
    let parsed = NpmRc::parse(&std::fs::read_to_string(user_npmrc).ok()?).ok()?;
    parsed.registry
}

impl NpmRc {
    /// Load user and project `.npmrc`; project values override user values.
    /// Đọc .npmrc từ đường dẫn (project) + ~/.npmrc (user) — project ghi đè user.
    pub fn load(project_dir: &Path) -> Result<Self> {
        let mut combined = Self::default();
        if let Some(user) = dirs::home_dir() {
            let user_npmrc = user.join(".npmrc");
            if user_npmrc.exists() {
                combined.merge(Self::parse(&std::fs::read_to_string(&user_npmrc)?)?);
            }
        }
        let project_npmrc = project_dir.join(".npmrc");
        if project_npmrc.exists() {
            // A planted symlink here would pull foreign files (e.g. another
            // user's tokens) into auth resolution — refuse, don't follow.
            // (Symlink trong project có thể kéo file ngoài vào auth.)
            if std::fs::symlink_metadata(&project_npmrc)
                .map(|metadata| metadata.file_type().is_symlink())
                .unwrap_or(false)
            {
                anyhow::bail!(
                    "project .npmrc '{}' must not be a symlink",
                    project_npmrc.display()
                );
            }
            combined.merge(Self::parse(&std::fs::read_to_string(&project_npmrc)?)?);
        }
        Ok(combined)
    }

    /// Merge another npmrc model with the incoming values taking precedence.
    /// Gộp model npmrc khác và ưu tiên các giá trị từ model được đưa vào.
    pub fn merge(&mut self, other: Self) {
        if other.registry.is_some() {
            self.registry = other.registry;
        }
        for (k, v) in other.scope_registries {
            self.scope_registries.insert(k, v);
        }
        for (k, v) in other.auth_tokens {
            self.auth_tokens.insert(k, v);
        }
        for (k, v) in other.basic_auth {
            self.basic_auth.insert(k, v);
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
