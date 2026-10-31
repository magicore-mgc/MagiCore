use crate::protocols::pypi::{
    InterpreterAbiFlavor, PypiFile, PypiProtocol, abi_compatible_for_interpreter, consumer_python,
    host_platform_version, platform_matches_host, platform_tag_matches_target,
    requires_python_allows,
};
use mgc_types::{MgError, MgResult};

impl PypiProtocol {
    /// Whether an artifact is a pure-Python wheel (no compiled ABI payload).
    /// (Artifact có phải wheel thuần Python — không có payload biên dịch.)
    pub fn is_importable_pure_wheel(artifact_url: &str) -> MgResult<bool> {
        let filename = Self::artifact_filename(artifact_url)?;
        Ok(Self::is_pure_wheel_filename(&filename))
    }

    /// Whether a wheel can be loaded by the declared Python runtime on this
    /// host. Pure wheels are always importable; a compiled wheel needs an
    /// explicit consumer version, otherwise the call fails closed naming the
    /// artifact instead of returning a bare "not importable".
    /// (Wheel có thể nạp trên host và Python runtime đã khai báo hay không.
    /// Wheel thuần luôn nạp được; wheel biên dịch phải có consumer rõ ràng.)
    pub fn is_importable_runtime_wheel(artifact_url: &str) -> MgResult<bool> {
        let filename = Self::artifact_filename(artifact_url)?;
        let Some(consumer) = consumer_python() else {
            // Pure wheels never need an ABI, so an unset consumer only
            // rejects ABI-specific artifacts — and says which one.
            // (Wheel thuần không cần ABI; thiếu consumer chỉ từ chối wheel ABI
            // và nêu rõ artifact nào.)
            if Self::is_pure_wheel_filename(&filename) {
                return Ok(true);
            }
            return Err(Self::missing_consumer_python_error(&filename));
        };
        Self::is_importable_wheel_for_resolution(&filename, consumer)
    }

    /// Filename-level check used by resolution with an already-known consumer.
    /// (Kiểm theo tên file với consumer đã biết — dùng trong resolve.)
    pub(in crate::protocols::pypi) fn is_importable_wheel_for_resolution(
        filename: &str,
        consumer: (u64, u64),
    ) -> MgResult<bool> {
        Ok(Self::is_importable_wheel_for_target(
            filename,
            std::env::consts::OS,
            std::env::consts::ARCH,
            Some(consumer),
            host_platform_version(),
        ))
    }

    /// Fail-closed error for an ABI-specific artifact when no consumer Python
    /// could be detected. Name the artifact so the message never blames a pure
    /// wheel, and point at the variable that fixes it.
    /// (Lỗi fail-closed khi không dò được consumer cho artifact ABI; nêu tên
    /// artifact và biến để sửa.)
    fn missing_consumer_python_error(filename: &str) -> MgError {
        MgError::Other(format!(
            "native Python install cannot materialize '{filename}': compiled wheels need the running interpreter's Python version, and no interpreter could be detected; set MGC_PYTHON_VERSION to the exact Python minor used by `mgc run`/`mgc dev`"
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
            && abi_compatible_for_interpreter(&py, &abi, major, minor, InterpreterAbiFlavor::Gil)
    }

    /// Pure-python wheel tags: `{py}-none-any` with a py2/py3 interpreter
    /// tag (compound `py2.py3` included).
    fn is_pure_wheel_filename(filename: &str) -> bool {
        let stem = filename.strip_suffix(".whl").unwrap_or(filename);
        let mut parts = stem.rsplit('-');
        let platform = parts.next().unwrap_or("");
        let abi = parts.next().unwrap_or("");
        let py = parts.next().unwrap_or("");
        let generic_interpreter = py.split('.').any(|tag| tag == "py3");
        platform == "any" && abi == "none" && generic_interpreter
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
    // With a declared consumer, prefer only a wheel the interpreter can load;
    // otherwise the old platform-only probe is kept (a recorded ABI warning
    // covers that historical path). Without this filter a `cp314` package
    // whose `cp314` wheel exists could still be handed a mismatched binary.
    // (Khi biết consumer, chỉ nhận wheel đúng ABI để loader nạp được; không
    // biết thì giữ probe theo platform cũ.)
    let host_wheel = |f: &&PypiFile| match consumer {
        Some(_) => is_importable_wheel_for_consumer(f, consumer),
        None => wheel_platform_matches_host(&f.filename),
    };
    if let Some(f) = files
        .iter()
        .filter(compatible)
        .find(|f| f.packagetype == "bdist_wheel" && host_wheel(f))
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

/// ABI-specific wheel whose platform and `(py, abi)` tags fit the consumer.
/// (Wheel ABI khớp cả platform lẫn consumer.)
fn is_importable_wheel_for_consumer(file: &PypiFile, consumer: Option<(u64, u64)>) -> bool {
    let Some((major, minor)) = consumer else {
        return false;
    };
    let (py, abi, plat) = wheel_tags(&file.filename);
    platform_tag_matches_target(
        &plat,
        std::env::consts::OS,
        std::env::consts::ARCH,
        host_platform_version(),
    ) && abi_compatible_for_interpreter(&py, &abi, major, minor, InterpreterAbiFlavor::Gil)
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
    abi_compatible_for_interpreter(&py, &abi, major, minor, InterpreterAbiFlavor::Gil)
}
