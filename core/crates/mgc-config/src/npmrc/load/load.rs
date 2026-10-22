//! User/project npmrc loading and merge / Nạp và gộp npmrc user/project.

use anyhow::Result;
use std::path::Path;

use crate::npmrc::NpmRc;

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
