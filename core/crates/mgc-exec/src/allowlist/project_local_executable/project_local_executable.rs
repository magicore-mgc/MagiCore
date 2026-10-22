use std::path::{Path, PathBuf};

/// Allow an explicitly addressed executable only when it resolves inside the project root.
/// Chỉ cho binary được gọi bằng path tường minh khi nó nằm trong project root.
pub(in crate::allowlist) fn is_project_local_executable(name: &str, project_root: &Path) -> bool {
    if !name.contains('/') && !name.contains('\\') {
        return false;
    }
    let requested = PathBuf::from(name);
    let root = match project_root.canonicalize() {
        Ok(root) => root,
        Err(_) => return false,
    };
    let candidate = if requested.is_absolute() {
        requested
    } else {
        project_root.join(requested)
    };
    let executable = match candidate.canonicalize() {
        Ok(executable) => executable,
        Err(_) => return false,
    };
    if !executable.starts_with(&root) {
        return false;
    }
    let metadata = match executable.metadata() {
        Ok(metadata) if metadata.is_file() => metadata,
        _ => return false,
    };

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o111 != 0
    }
    #[cfg(windows)]
    {
        let _ = metadata;
        executable
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("exe"))
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = metadata;
        false
    }
}
