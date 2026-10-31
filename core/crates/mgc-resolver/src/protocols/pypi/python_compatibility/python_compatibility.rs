//! ABI, interpreter-flavor and `requires_python` compatibility predicates.
//! Predicate tương thích ABI, flavor interpreter và `requires_python`.

use mgc_types::{MgError, MgResult};
use std::path::Path;

/// Environment variable that overrides the detected consumer interpreter.
/// Biến môi trường ghi đè consumer interpreter đã dò.
pub const PYTHON_VERSION_ENV: &str = "MGC_PYTHON_VERSION";

/// Inline program printed by the probe: the interpreter's own version.
/// Chương trình inline để probe in ra version của chính interpreter.
const INTERPRETER_VERSION_PROGRAM: &str =
    "import sys; print(f'{sys.version_info[0]}.{sys.version_info[1]}')";

/// Allowlisted Python launchers, in the order `mgc run`/`mgc dev` prefer them
/// (macOS ships only `python3`; setup-python CI images provide `python`).
/// Launcher Python trong allowlist, theo thứ tự `mgc run`/`mgc dev` ưu tiên.
const PYTHON_LAUNCHERS: &[&str] = &["python", "python3"];

/// Probing an interpreter is a read-only `--version`-class query, so it gets a
/// short fixed ceiling instead of the generic tool timeout.
/// Probe interpreter là truy vấn chỉ đọc, nên dùng hạn ngắn cố định.
const PROBE_TIMEOUT_SECS: u64 = 10;

/// Consumer Python for wheel ABI selection.
/// Python consumer để chọn wheel ABI.
///
/// Resolution order: `MGC_PYTHON_VERSION` override → the interpreter the
/// Python lane actually launches (`python`, then `python3`) → unknown.
/// Unknown (`None`, only reachable when the override is unset AND no launcher
/// answers) keeps the historical lenient `requires_python` behavior while
/// ABI-specific artifacts stay refused by the caller, naming the artifact and
/// the variable that fixes it.
/// (Thứ tự: env ghi đè → interpreter lane Python thật sự khởi chạy → chưa
/// biết. Chưa biết chỉ xảy ra khi không có env VÀ không launcher nào trả lời;
/// artifact ABI vẫn bị từ chối ở caller kèm tên artifact và biến để sửa.)
pub(in crate::protocols::pypi) fn consumer_python() -> Option<(u64, u64)> {
    detected_interpreter_python().ok().flatten()
}

/// Detect the consumer interpreter, naming what went wrong when nothing could
/// be probed. An unset override NEVER yields a guessed ABI: it either resolves
/// the real interpreter or fails with the reason.
/// Dò consumer interpreter, nêu lý do khi không probe được. Thiếu env KHÔNG
/// bao giờ dẫn tới ABI đoán: hoặc ra interpreter thật, hoặc lỗi kèm lý do.
///
/// Launchers are tried in lane order. A launcher that is not installed is the
/// expected macOS case (`python` is absent, `python3` present), so a missing
/// launcher moves to the next one; any other executor failure (allowlist or
/// policy refusal, timeout) is reported because it is not "unknown".
/// (Dò theo thứ tự lane. Launcher chưa cài là trường hợp bình thường trên
/// macOS nên chuyển launcher kế tiếp; lỗi executor khác thì báo lỗi.)
pub(in crate::protocols::pypi) fn detected_interpreter_python() -> MgResult<Option<(u64, u64)>> {
    if std::env::var_os(PYTHON_VERSION_ENV).is_some() {
        return Ok(python_version_override());
    }
    for launcher in PYTHON_LAUNCHERS {
        match probe_interpreter(Path::new(launcher))? {
            Some(version) => return Ok(Some(version)),
            None => continue,
        }
    }
    Err(MgError::Other(format!(
        "cannot detect the running Python interpreter version: neither `{}` answered `-c \"import sys; ...\"` with a major.minor; set {PYTHON_VERSION_ENV} to the exact Python minor used by `mgc run`/`mgc dev`",
        PYTHON_LAUNCHERS.join("` nor `")
    )))
}

/// The explicit override, when set and parseable (legacy compatibility).
/// Env ghi đè tường minh khi được đặt và parse được (tương thích cũ).
fn python_version_override() -> Option<(u64, u64)> {
    std::env::var_os(PYTHON_VERSION_ENV)
        .as_deref()
        .and_then(|raw| parse_python_version(&raw.to_string_lossy()))
}

/// Probe one launcher through the allowlisted executor and parse its version.
/// Probe một launcher qua executor allowlist và parse version của nó.
///
/// The probe is the same allowlisted lane contract `mgc run`/`mgc dev` use, so
/// the resolver never spawns an unvetted process and never reads a version
/// from anywhere but the interpreter itself. An executor refusal (for example
/// an allowlist/policy denial) is a real reason, not "unknown".
/// (Probe dùng đúng contract lane allowlist của `mgc run`/`mgc dev`, nên
/// resolver không spawn process chưa kiểm và chỉ đọc version từ chính
/// interpreter. Bị executor từ chối là lý do thật, không phải "chưa biết".)
fn probe_interpreter(launcher: &Path) -> MgResult<Option<(u64, u64)>> {
    let options = mgc_exec::prelude::ExecOptions {
        timeout: Some(std::time::Duration::from_secs(PROBE_TIMEOUT_SECS)),
        execution_scope: Some(mgc_exec::prelude::ExecutionScope::TestRunner),
        ..Default::default()
    };
    let arguments = vec!["-c".to_string(), INTERPRETER_VERSION_PROGRAM.to_string()];
    // The program name is the allowlisted launcher literal itself (the caller
    // only ever passes `PYTHON_LAUNCHERS` entries), so the executor sees a
    // fixed command, never a path interpolated from input.
    // (Tên chương trình chính là literal launcher trong allowlist — caller chỉ
    // truyền entry `PYTHON_LAUNCHERS` — nên executor thấy lệnh cố định.)
    let program = launcher.to_string_lossy();
    let report = match mgc_exec::prelude::run(&program, &arguments, &options) {
        Ok(report) => report,
        // Not installed (`python` on macOS, minimal containers): try the next
        // launcher instead of turning a normal absence into a hard failure.
        // (Chưa cài: chuyển launcher kế tiếp thay vì biến việc bình thường
        // thành lỗi cứng.)
        Err(error) if is_launcher_missing(&error) => return Ok(None),
        Err(error) => {
            return Err(MgError::Other(format!(
                "cannot probe Python interpreter `{}`: {error}",
                launcher.display()
            )));
        }
    };
    if report.exit_code != 0 || report.dry_run {
        return Ok(None);
    }
    // stdout_full is only populated when capture was requested; the default
    // capture path fills stdout_tail, so prefer the full payload.
    // (`stdout_full` chỉ có khi bật capture; mặc định dùng `stdout_tail`.)
    let stdout = if report.stdout_full.trim().is_empty() {
        &report.stdout_tail
    } else {
        &report.stdout_full
    };
    Ok(stdout.lines().find_map(parse_python_version))
}

/// Whether a spawn failure means the launcher is simply not installed.
///
/// The executor wraps the OS error with context, so the check is on the
/// rendered message: a missing program surfaces as "No such file or
/// directory" (Unix) / "The system cannot find the file" (Windows), while an
/// allowlist or policy refusal has unrelated wording and must NOT be read as
/// "skip this launcher".
/// (Lỗi spawn chỉ là "chưa cài" khi thông điệp là lỗi thiếu file của OS; bị
/// allowlist/policy từ chối có câu chữ khác nên không được coi là bỏ qua.)
fn is_launcher_missing(error: &anyhow::Error) -> bool {
    let rendered = format!("{error:#}").to_ascii_lowercase();
    rendered.contains("no such file or directory")
        || rendered.contains("cannot find the file")
        || rendered.contains("program not found")
}

/// Parse a major/minor pair from text such as `3.14`, `3.14.8`, or `314`.
/// Trả `(major, minor)` từ chuỗi kiểu `3.14`, `3.14.8`, hoặc `314`.
///
/// `314` (no dot) means 3.14: the first digit is the major version and the
/// rest is the minor — the same convention `MGC_PYTHON_VERSION` accepts.
/// (`314` không dấu chấm nghĩa là 3.14: chữ số đầu là major, phần còn lại là
/// minor — cùng quy ước mà `MGC_PYTHON_VERSION` chấp nhận.)
pub(in crate::protocols::pypi) fn parse_python_version(text: &str) -> Option<(u64, u64)> {
    let digits: String = text
        .trim()
        .chars()
        .take_while(|character| character.is_ascii_digit() || *character == '.')
        .collect();
    if digits.is_empty() {
        return None;
    }
    let parts: Vec<&str> = digits.split('.').filter(|part| !part.is_empty()).collect();
    match parts.as_slice() {
        // `39` (no dot) = 3.9; `310` = 3.10 (first digit = major).
        // (`39` không dấu chấm là 3.9.)
        [single] if !single.contains('.') && single.len() >= 2 => {
            let major: u64 = single[..1].parse().ok()?;
            let minor: u64 = single[1..].parse().ok()?;
            (major > 0).then_some((major, minor))
        }
        [major, minor, ..] => {
            let major: u64 = major.parse().ok()?;
            let minor: u64 = minor.parse().ok()?;
            (major > 0).then_some((major, minor))
        }
        [major] => {
            let major: u64 = major.parse().ok()?;
            (major > 0).then_some((major, 0))
        }
        _ => None,
    }
}

/// Interpreter ABI flavor used for binary-wheel selection.
///
/// `GIL` is the only flavor a build can prove here. Free-threaded CPython
/// (`cp314t`) needs its own `t`-suffixed ABI and cannot load abi3/GIL
/// binaries, so the flavor is never inferred from the version alone: an
/// unknown runtime falls back to `GIL`, the historical behavior.
/// (Flavor ABI dùng để chọn wheel binary. Chỉ `GIL` là chứng minh được ở đây;
/// free-threaded cần ABI `t` riêng nên không suy ra từ version.)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InterpreterAbiFlavor {
    Gil,
    FreeThreaded,
}

impl InterpreterAbiFlavor {
    /// Tag suffix appended to the `cp<major><minor>` ABI tag.
    /// (Hậu tố gắn vào tag ABI `cp<major><minor>`.)
    fn abi_suffix(self) -> &'static str {
        match self {
            Self::Gil => "",
            Self::FreeThreaded => "t",
        }
    }
}

/// Flavor-aware ABI compatibility; `flavor` is the running interpreter's ABI.
/// (Tương thích ABI có xét flavor của interpreter.)
pub(crate) fn abi_compatible_for_interpreter(
    py: &str,
    abi: &str,
    major: u64,
    minor: u64,
    flavor: InterpreterAbiFlavor,
) -> bool {
    if abi == "none" {
        // `py3-none-any` universal, or versioned pure-python (`cp39-none-any`).
        return py.split('.').any(|tag| {
            tag == "py3" || tag == format!("py{major}{minor}") || tag == format!("cp{major}{minor}")
        });
    }
    if abi == "abi3" {
        // Stable ABI: `cp37-abi3` needs consumer >= 3.7. abi3 is a GIL-ABI
        // contract, so a free-threaded interpreter cannot load it.
        // (ABI ổn định: chạy trên mọi bản >= floor; abi3 thuộc ABI GIL nên
        // interpreter free-threaded không nạp được.)
        if flavor == InterpreterAbiFlavor::FreeThreaded {
            return false;
        }
        for tag in py.split('.') {
            if let Some(rest) = tag.strip_prefix("cp") {
                let floor: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
                let (f_major, f_minor) = match floor.len() {
                    0 => (0, 0),
                    1 => (floor.parse().unwrap_or(0), 0),
                    _ => (
                        floor[..1].parse().unwrap_or(0),
                        floor[1..].parse().unwrap_or(0),
                    ),
                };
                if major > f_major || (major == f_major && minor >= f_minor) {
                    return true;
                }
            }
        }
        return false;
    }
    // Versioned ABI (`cp39`, `cp314t`, `pp39`): exact match only, including
    // the free-threaded `t` suffix. A `t`-suffixed interpreter tag is binary
    // incompatible with the GIL build of the same micro version.
    // (ABI theo version: khớp chính xác, kể cả hậu tố `t`; bản `t` không thể
    // nạp binary của bản GIL cùng version.)
    let want = format!("cp{major}{minor}{}", flavor.abi_suffix());
    py.split('.').any(|tag| tag == want) && abi == want
}

/// Does a `requires_python` specifier allow consumer `(major, minor)`?
/// Comma = AND; operators `==` (with `.*`), `>=`, `<=`, `>`, `<`, `!=`,
/// `~=` (compatible release). Unknown/empty parts fail closed (deny).
/// (`requires_python` có cho consumer không? Không rõ → từ chối.)
pub(in crate::protocols::pypi) fn requires_python_allows(
    spec: &str,
    major: u64,
    minor: u64,
) -> bool {
    fn parse_ver(text: &str) -> Option<(u64, u64, u64)> {
        let text = text.trim().trim_end_matches(".*");
        let mut parts = text.split('.');
        let maj: u64 = parts.next()?.trim().parse().ok()?;
        let min: u64 = parts.next().unwrap_or("0").trim().parse().ok()?;
        let pat: u64 = parts.next().unwrap_or("0").trim().parse().ok()?;
        Some((maj, min, pat))
    }
    let consumer = (major, minor, 0u64);
    for part in spec.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        let (op, ver) = if let Some(v) = part.strip_prefix("==") {
            ("==", v)
        } else if let Some(v) = part.strip_prefix("~=") {
            ("~=", v)
        } else if let Some(v) = part.strip_prefix("!=") {
            ("!=", v)
        } else if let Some(v) = part.strip_prefix(">=") {
            (">=", v)
        } else if let Some(v) = part.strip_prefix("<=") {
            ("<=", v)
        } else if let Some(v) = part.strip_prefix('>') {
            (">", v)
        } else if let Some(v) = part.strip_prefix('<') {
            ("<", v)
        } else {
            return false;
        };
        let wildcard = ver.trim().ends_with(".*");
        let Some(target) = parse_ver(ver) else {
            return false;
        };
        let ok = match op {
            "==" => {
                if wildcard {
                    let prefix = ver.trim().trim_end_matches(".*").trim_end_matches('.');
                    let want: Vec<u64> = prefix
                        .split('.')
                        .filter_map(|p| p.trim().parse().ok())
                        .collect();
                    let have = [consumer.0, consumer.1, consumer.2];
                    !want.is_empty()
                        && want.len() <= 3
                        && want.iter().enumerate().all(|(i, w)| have[i] == *w)
                } else {
                    consumer == target
                }
            }
            "!=" => {
                if wildcard {
                    let prefix = ver.trim().trim_end_matches(".*").trim_end_matches('.');
                    let want: Vec<u64> = prefix
                        .split('.')
                        .filter_map(|p| p.trim().parse().ok())
                        .collect();
                    want.is_empty()
                        || !(want.len() <= 3
                            && want
                                .iter()
                                .enumerate()
                                .all(|(i, w)| [consumer.0, consumer.1, consumer.2][i] == *w))
                } else {
                    consumer != target
                }
            }
            ">=" => consumer >= target,
            "<=" => consumer <= target,
            ">" => consumer > target,
            "<" => consumer < target,
            "~=" => {
                // Compatible release: >= V, == V.* (same prefix length).
                // (Release tương thích.)
                let dots = ver.trim().matches('.').count();
                if dots == 0 {
                    return false;
                }
                let prefix_len = dots;
                let have = [consumer.0, consumer.1, consumer.2];
                let want = [target.0, target.1, target.2];
                consumer >= target && have[..prefix_len] == want[..prefix_len]
            }
            _ => return false,
        };
        if !ok {
            return false;
        }
    }
    true
}
