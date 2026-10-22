//! Locate Flutter executable candidates without spawning processes.
//! Tìm candidate executable Flutter mà không spawn process.

use std::ffi::OsStr;
use std::path::PathBuf;

/// Locate a Flutter executable without invoking it or another package tool.
/// On Windows, honor PATHEXT so flutter.cmd/flutter.bat installations work.
/// (Chỉ dò filesystem, không chạy Flutter hay package manager; Windows theo PATHEXT.)
pub(crate) fn find_flutter_executable_in_path(
    path: &OsStr,
    pathext: Option<&OsStr>,
    windows: bool,
) -> Option<PathBuf> {
    let extensions = if windows {
        let configured = pathext
            .map(|value| value.to_string_lossy().into_owned())
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| ".COM;.EXE;.BAT;.CMD".to_string());
        configured
            .split(';')
            .map(str::trim)
            .filter(|extension| !extension.is_empty())
            .map(str::to_string)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    for directory in std::env::split_paths(path) {
        let mut candidates = vec![directory.join("flutter")];
        if windows {
            candidates.extend(extensions.iter().map(|extension| {
                let mut name = std::ffi::OsString::from("flutter");
                name.push(extension);
                directory.join(name)
            }));
        }
        for candidate in candidates {
            if candidate.is_file()
                && let Ok(canonical) = candidate.canonicalize()
            {
                return Some(canonical);
            }
        }
    }
    None
}
