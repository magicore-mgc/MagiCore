//! Validated npm auth-token persistence / Lưu auth token npm sau khi kiểm tra.

use anyhow::Result;
use std::path::Path;

use crate::npmrc::NpmRc;

impl NpmRc {
    /// Ghi `//host/:_authToken=TOKEN` vào file .npmrc (thay dòng cũ nếu có).
    /// (login flow — lưu token sau `mgc login` / `mgc registry user add`)
    ///
    /// Security: refuses symlinked paths (a planted link would redirect a
    /// live token outside the project) and creates the file `0600` on
    /// Unix, mirroring the keyring writer. Existing files keep their
    /// content; only the token line is replaced.
    /// (Từ chối path symlink; file mới `0600` trên Unix như keyring.)
    pub fn save_auth_token(npmrc_path: &Path, host: &str, token: &str) -> Result<()> {
        let registry_url = format!("https://{host}/");
        Self::save_auth_token_for_registry_url(npmrc_path, &registry_url, token)
    }

    /// Save a token with the registry's host and path scope instead of widening it to the full host.
    /// Lưu token theo cả host lẫn path để không mở rộng credential sang registry khác cùng host.
    pub fn save_auth_token_for_registry_url(
        npmrc_path: &Path,
        registry_url: &str,
        token: &str,
    ) -> Result<()> {
        use std::fs;
        use std::io::Write;

        let mut scope = url::Url::parse(registry_url)
            .map_err(|_| anyhow::anyhow!("invalid registry URL for auth token scope"))?;
        if !matches!(scope.scheme(), "http" | "https")
            || scope.host_str().is_none()
            || !scope.username().is_empty()
            || scope.password().is_some()
            || scope.query().is_some()
            || scope.fragment().is_some()
        {
            anyhow::bail!("invalid registry URL for auth token scope");
        }
        if token.is_empty()
            || token
                .bytes()
                .any(|byte| byte.is_ascii_whitespace() || byte.is_ascii_control())
        {
            anyhow::bail!("invalid registry auth token");
        }
        let mut path = scope.path().to_owned();
        if !path.ends_with('/') {
            path.push('/');
        }
        scope.set_path(&path);
        let scoped_authority = scope
            .as_str()
            .strip_prefix("https:")
            .or_else(|| scope.as_str().strip_prefix("http:"))
            .ok_or_else(|| anyhow::anyhow!("invalid registry URL for auth token scope"))?;
        let auth_scope = format!("{scoped_authority}:_authToken");

        if fs::symlink_metadata(npmrc_path)
            .map(|metadata| metadata.file_type().is_symlink())
            .unwrap_or(false)
        {
            anyhow::bail!(
                "refusing to write auth token through symlink '{}'",
                npmrc_path.display()
            );
        }

        let new_line = format!("{auth_scope}={token}");

        let mut lines: Vec<String> = Vec::new();
        if npmrc_path.exists() {
            let content = fs::read_to_string(npmrc_path)?;
            let mut replaced = false;
            for raw in content.lines() {
                let line = raw.trim_end();
                if line.starts_with(&format!("{auth_scope}=")) {
                    lines.push(new_line.clone());
                    replaced = true;
                } else {
                    lines.push(line.to_string());
                }
            }
            if !replaced {
                lines.push(new_line);
            }
        } else {
            lines.push(new_line);
        }

        let mut out = String::new();
        for line in lines {
            out.push_str(&line);
            out.push('\n');
        }
        let mut options = fs::OpenOptions::new();
        options.write(true).create(true).truncate(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut f = options.open(npmrc_path)?;
        f.write_all(out.as_bytes())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = std::fs::Permissions::from_mode(0o600);
            std::fs::set_permissions(npmrc_path, permissions)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
