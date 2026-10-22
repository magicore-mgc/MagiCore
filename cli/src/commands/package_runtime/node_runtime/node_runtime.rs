//! Node script detection, host runtime lookup and quoted launcher creation.
//! Nhận diện script Node, tìm runtime host và tạo launcher có quoting an toàn.

use anyhow::Result;
use std::path::{Path, PathBuf};

pub(in crate::commands::package_runtime) fn is_node_script(path: &Path) -> bool {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case("js")
                || extension.eq_ignore_ascii_case("mjs")
                || extension.eq_ignore_ascii_case("cjs")
        })
}

pub(in crate::commands::package_runtime) fn resolve_host_node_binary(
    path: &std::ffi::OsStr,
) -> Result<PathBuf> {
    for directory in std::env::split_paths(path).filter(|directory| directory.is_absolute()) {
        #[cfg(windows)]
        let candidates = [directory.join("node.exe")];
        #[cfg(not(windows))]
        let candidates = [directory.join("node")];
        for candidate in candidates {
            let Ok(metadata) = std::fs::metadata(&candidate) else {
                continue;
            };
            if !metadata.is_file() {
                continue;
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if metadata.permissions().mode() & 0o111 == 0 {
                    continue;
                }
            }
            return candidate.canonicalize().map_err(Into::into);
        }
    }
    Err(super::registry_package_missing_host_node())
}

pub(in crate::commands::package_runtime) fn create_node_launcher(
    execution_cwd: &Path,
    entrypoint: &Path,
) -> Result<tempfile::NamedTempFile> {
    let entrypoint = entrypoint.to_string_lossy();
    let entrypoint_json = serde_json::to_string(entrypoint.as_ref())?;
    // Use Node's entrypoint dispatcher so CommonJS receives require.main and
    // ESM retains native module-type detection and top-level await semantics.
    // Dùng dispatcher entrypoint của Node để giữ require.main cho CommonJS
    // và nhận diện module/top-level await đúng với ESM.
    let source = format!(
        "import Module from 'node:module';\nconst entry = {entrypoint_json};\nprocess.argv[1] = entry;\nModule.runMain(entry);\n"
    );
    let mut launcher = tempfile::Builder::new()
        .prefix(".mgc-create-initializer-")
        .suffix(".mjs")
        .tempfile_in(execution_cwd)?;
    std::io::Write::write_all(&mut launcher, source.as_bytes())?;
    Ok(launcher)
}
#[cfg(test)]
#[path = "test/node_runtime.rs"]
mod tests;
