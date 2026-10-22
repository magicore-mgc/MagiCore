use crate::protocols::pypi::{
    PypiFile, PypiProtocol, abi_compatible, consumer_python, host_platform_version,
    platform_matches_host, platform_tag_matches_target, requires_python_allows,
};
use mgc_types::MgResult;

impl PypiProtocol {
    /// Whether an artifact can be materialized by the current native
    /// Python runtime (pure-Python wheel only; no build backend or ABI
    /// loader is implemented yet).
    /// (Artifact có thể materialize bởi runtime Python native hiện tại.)
    pub fn is_importable_pure_wheel(artifact_url: &str) -> MgResult<bool> {
        let filename = Self::artifact_filename(artifact_url)?;
        Ok(Self::is_pure_wheel_filename(&filename))
    }

    /// Whether a wheel can be loaded by the declared Python runtime on this
    /// host. Compiled wheels require an explicit consumer version so resolver,
    /// installer, and runner cannot silently disagree about the ABI.
    /// (Wheel có thể nạp trên host và Python runtime đã khai báo hay không.)
    pub fn is_importable_runtime_wheel(artifact_url: &str) -> MgResult<bool> {
        let filename = Self::artifact_filename(artifact_url)?;
        if Self::is_pure_wheel_filename(&filename) {
            return Ok(true);
        }
        let Some(consumer) = consumer_python() else {
            return Ok(false);
        };
        Ok(Self::is_importable_wheel_for_target(
            &filename,
            std::env::consts::OS,
            std::env::consts::ARCH,
            Some(consumer),
            host_platform_version(),
        ))
    }

    /// Deterministic wheel compatibility predicate shared with cross-platform
    /// tests. A missing Python minor intentionally rejects ABI-specific wheels.
    /// (Predicate tương thích ổn định cho test và bộ chọn wheel đa nền tảng.)
    pub fn is_importable_wheel_for_target(
        filename: &str,
        os: &str,
        arch: &str,
        consumer: Option<(u64, u64)>,
        platform_version: Option<(u64, u64)>,
    ) -> bool {
        if Self::is_pure_wheel_filename(filename) {
            return true;
        }
        let Some((major, minor)) = consumer else {
            return false;
        };
        let (py, abi, plat) = wheel_tags(filename);
        platform_tag_matches_target(&plat, os, arch, platform_version)
            && abi_compatible(&py, &abi, major, minor)
    }

    /// Pure-python wheel tags: `{py}-none-any` with a py2/py3 interpreter
    /// tag (compound `py2.py3` included).
    fn is_pure_wheel_filename(filename: &str) -> bool {
        let stem = filename.strip_suffix(".whl").unwrap_or(filename);
        let mut parts = stem.rsplit('-');
        let platform = parts.next().unwrap_or("");
        let abi = parts.next().unwrap_or("");
        let py = parts.next().unwrap_or("");
        platform == "any"
            && abi == "none"
            && (py == "py" || py.starts_with("py2") || py.starts_with("py3"))
    }
}

/// Select the best installable file: universal wheel → host wheel →
/// sdist. There is deliberately NO "any wheel" fallback: installing a
/// wheel built for another platform is a silent wrong-artifact install
/// (V1.2: fail-closed — no file at all beats the wrong file).
/// Chọn file cài tốt nhất: wheel phổ quát → wheel host → sdist. Cố ý
/// KHÔNG fallback "wheel bất kỳ".
pub(in crate::protocols::pypi) fn select_file(
    files: &[PypiFile],
    consumer: Option<(u64, u64)>,
) -> Option<&PypiFile> {
    let compatible = |file: &&PypiFile| file_requires_python_allows(file, consumer);
    if let Some(f) = files
        .iter()
        .filter(compatible)
        .find(|f| is_universal_wheel(f))
    {
        return Some(f);
    }
    if let Some(f) = files
        .iter()
        .filter(compatible)
        .find(|f| f.packagetype == "bdist_wheel" && wheel_platform_matches_host(&f.filename))
    {
        return Some(f);
    }
    files
        .iter()
        .filter(compatible)
        .find(|f| f.packagetype == "sdist")
}

/// Ignore a file whose `Requires-Python` excludes the selected consumer.
/// Keep the historical unresolved-runtime behavior until runtime discovery is
/// wired into the project resolver rather than inventing a target version.
/// Bỏ file có `Requires-Python` loại interpreter mục tiêu; khi chưa biết runtime,
/// giữ hành vi cũ thay vì tự bịa phiên bản đích.
pub(in crate::protocols::pypi) fn file_requires_python_allows(
    file: &PypiFile,
    consumer: Option<(u64, u64)>,
) -> bool {
    match (file.requires_python.as_deref(), consumer) {
        (Some(required), Some((major, minor))) => {
            required.trim().is_empty() || requires_python_allows(required, major, minor)
        }
        _ => true,
    }
}

pub(in crate::protocols::pypi) fn is_universal_wheel(file: &PypiFile) -> bool {
    if file.packagetype != "bdist_wheel" {
        return false;
    }
    let (py, abi, plat) = wheel_tags(&file.filename);
    abi == "none" && plat == "any" && py.contains("py3")
}

/// Parse a wheel filename into `(python, abi, platform)` tags (last 3 parts).
/// Parse filename wheel thành tag `(python, abi, platform)` (3 phần cuối).
pub(in crate::protocols::pypi) fn wheel_tags(filename: &str) -> (String, String, String) {
    let stem = filename.strip_suffix(".whl").unwrap_or(filename);
    let parts: Vec<&str> = stem.split('-').collect();
    if parts.len() >= 3 {
        let n = parts.len();
        (
            parts[n - 3].to_string(),
            parts[n - 2].to_string(),
            parts[n - 1].to_string(),
        )
    } else {
        (String::new(), String::new(), String::new())
    }
}

pub(in crate::protocols::pypi) fn wheel_platform_matches_host(filename: &str) -> bool {
    let (py, abi, plat) = wheel_tags(filename);
    if !platform_matches_host(&plat) {
        return false;
    }
    // Without a known consumer Python (`MGC_PYTHON_VERSION`), any
    // platform-matching wheel is accepted (historical behavior) — the
    // chosen ABI is recorded in markers + warned about at install so a
    // cp310-on-3.9 style mismatch is never silent. With the env set,
    // only ABI-compatible wheels pass (exact cpXY / abi3 floor /
    // universal); the rest are skipped, never installed.
    // (Không biết Python consumer thì nhận wheel khớp platform (cũ) +
    // cảnh báo; có env thì lọc ABI chặt.)
    let Some((major, minor)) = consumer_python() else {
        return true;
    };
    abi_compatible(&py, &abi, major, minor)
}
