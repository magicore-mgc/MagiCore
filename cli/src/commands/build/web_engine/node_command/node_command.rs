use anyhow::Result;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub(crate) fn node_runner() -> PathBuf {
    PathBuf::from("node")
}

pub(crate) fn node_bin_args(
    project_root: &Path,
    bin_name: &str,
    args: &[&str],
) -> Result<Vec<OsString>> {
    let bin_dir = project_root.join("node_modules").join(".bin");
    let bin = bin_dir.join(bin_name);
    // Windows: npm-style .bin uses .cmd shims (create_bin_link writes them) —
    // the extensionless entry may not exist. Accept the shim variants.
    // Windows: .bin dùng shim .cmd — file gốc có thể không tồn tại, chấp nhận biến thể.
    let entry_path = if bin.exists() {
        bin.clone()
    } else {
        let shim = bin_dir.join(format!("{bin_name}.cmd"));
        if shim.exists() {
            shim
        } else {
            return Err(anyhow::Error::new(
                super::error::NodeCommandError::MissingLocalExecutable {
                    name: bin_name.to_string(),
                    project_root: project_root.to_path_buf(),
                },
            ));
        }
    };

    // Resolve symlink target when the entry is a real symlink (unix). On
    // Windows the npm .cmd shim wraps the real JS entry — parse the quoted
    // script path from the shim body (npm shims end with: "<js path>" %*)
    // so node receives the JS file, not the shim itself.
    // Unix: đọc đích symlink. Windows: shim .cmd bọc JS thật — trích đường
    // dẫn JS trong nháy kép từ thân shim để node chạy JS, không chạy shim.
    let entry = {
        #[cfg(unix)]
        {
            std::fs::read_link(&entry_path)
                .map(|target| {
                    if target.is_absolute() {
                        target
                    } else {
                        entry_path.parent().unwrap_or(project_root).join(target)
                    }
                })
                .unwrap_or_else(|_| entry_path.clone())
        }
        #[cfg(not(unix))]
        {
            let raw = std::fs::read_to_string(&entry_path).unwrap_or_default();
            let mut resolved: Option<PathBuf> = None;
            for line in raw.lines().rev() {
                // mgc/npm shim target line:  "...path..." %* — the quoted path
                // may be extensionless (typescript's bin/tsc), so accept any
                // quoted existing file on the last command line.
                // Dòng đích shim: "...đường dẫn..." %* — đích có thể không
                // đuôi file (bin/tsc), nhận mọi đường dẫn tồn tại trong nháy.
                if let Some(q1) = line.find('"')
                    && let Some(q2) = line[q1 + 1..].find('"')
                {
                    let candidate = PathBuf::from(&line[q1 + 1..q1 + 1 + q2]);
                    if candidate.is_file() {
                        resolved = Some(candidate);
                        break;
                    }
                }
            }
            match resolved {
                Some(path) if path.is_absolute() => path,
                Some(path) => entry_path.parent().unwrap_or(project_root).join(path),
                None => entry_path.clone(),
            }
        }
    };

    let mut result = vec![
        OsString::from("--preserve-symlinks"),
        OsString::from("--preserve-symlinks-main"),
        entry.into_os_string(),
    ];
    result.extend(args.iter().map(OsString::from));
    Ok(result)
}

/// Plain variant WITHOUT --preserve-symlinks*: beasties (Angular
/// critical-CSS inliner) loads its DOM implementation twice under
/// symlink-preserving resolution on store-backed trees, so the document
/// and its elements come from different module copies
/// (`document.documentElement?.setAttribute is not a function`). Realpath
/// resolution unifies the copies — the same invocation npm/npx performs.
/// Only frameworks proven broken under the flags may use this.
/// (Biến thể không preserve-symlinks — chỉ framework đã chứng minh vỡ.)
pub(crate) fn node_bin_args_plain(
    project_root: &Path,
    bin_name: &str,
    args: &[&str],
) -> Result<Vec<OsString>> {
    let mut full = node_bin_args(project_root, bin_name, args)?;
    full.retain(|arg| arg != "--preserve-symlinks" && arg != "--preserve-symlinks-main");
    Ok(full)
}

pub(crate) fn prepend_path(local_bin: &Path) -> Result<OsString> {
    let current = std::env::var_os("PATH").unwrap_or_default();
    let mut parts = vec![local_bin.as_os_str().to_os_string()];
    parts.extend(std::env::split_paths(&current).map(|path| path.into_os_string()));
    std::env::join_paths(parts)
        .map_err(|source| anyhow::Error::new(super::error::NodeCommandError::JoinPaths { source }))
}
