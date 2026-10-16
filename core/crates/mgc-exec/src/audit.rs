//! Audit log — ghi mỗi lần chạy exec (00-index §5.4)
//! (exec audit vào .magicore/exec.log — args đã REDACTED, không bao giờ chứa secret)

use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

// Serialize audit appends from parallel executor tasks within this process.
// Tuần tự hóa append audit giữa các task executor chạy song song trong process.
static APPEND_LOCK: Mutex<()> = Mutex::new(());

/// Một dòng audit (JSON) — args nhận SẴN đã redact từ sanitizer.
#[derive(Debug, Clone, Serialize)]
pub struct AuditEntry {
    pub cmd: String,
    pub args: Vec<String>,
    pub cwd: String,
    pub exit_code: i32,
    pub duration_ms: u64,
    pub dry_run: bool,
    pub ts: u64,
}

/// Append một dòng audit vào `path` (tạo parent dir nếu thiếu).
pub fn append(path: &Path, entry: &AuditEntry) -> anyhow::Result<()> {
    append_serialized(path, entry)
}

/// Record a user-selected compatibility process-tree scope before spawn.
/// Ghi phạm vi cây tiến trình compat do user chọn trước khi spawn.
pub fn append_compat_scope(
    path: &Path,
    manager: &str,
    root_cmd: &str,
    root_args: &[String],
    cwd: &Path,
    dry_run: bool,
) -> anyhow::Result<()> {
    #[derive(Serialize)]
    struct CompatScopeEntry<'a> {
        event: &'static str,
        manager: &'a str,
        root_cmd: &'a str,
        root_args: &'a [String],
        cwd: String,
        scope: &'static str,
        dry_run: bool,
        ts: u64,
    }

    append_serialized(
        path,
        &CompatScopeEntry {
            event: "compat_runner_scope",
            manager,
            root_cmd,
            root_args,
            cwd: cwd.display().to_string(),
            scope: "root_process_tree",
            dry_run,
            ts: now_ts(),
        },
    )
}

/// Record a package-manager process observed inside the declared compat tree.
/// Ghi process package manager được quan sát trong cây compat đã khai báo.
pub fn append_compat_child(
    path: &Path,
    manager: &str,
    pid: u32,
    image: &str,
    program: &str,
    args: &[String],
    cwd: &Path,
) -> anyhow::Result<()> {
    #[derive(Serialize)]
    struct CompatChildEntry<'a> {
        event: &'static str,
        manager: &'a str,
        pid: u32,
        image: &'a str,
        program: &'a str,
        args: &'a [String],
        cwd: String,
        scope: &'static str,
        ts: u64,
    }

    append_serialized(
        path,
        &CompatChildEntry {
            event: "compat_runner_child_observed",
            manager,
            pid,
            image,
            program,
            args,
            cwd: cwd.display().to_string(),
            scope: "root_process_tree",
            ts: now_ts(),
        },
    )
}

/// Record a native-mode descendant rejected for dependency mutation.
/// Only the classifier's redacted invocation shape is stored for the child;
/// raw child arguments never enter this event.
/// (Ghi descendant bị từ chối vì mutation dependency trong native mode.
/// Chỉ lưu hình dạng invocation đã che dữ liệu; không ghi raw child args.)
pub(crate) struct NativeChildDenial<'a> {
    pub root_cmd: &'a str,
    pub root_args: &'a [String],
    pub cwd: &'a Path,
    pub child_pid: u32,
    pub child_name: &'a str,
    pub child_image: &'a str,
    pub matched_by: &'a str,
    pub matched_operation: &'a str,
}

pub(crate) fn append_native_child_denial(
    path: &Path,
    denial: NativeChildDenial<'_>,
) -> anyhow::Result<()> {
    #[derive(Serialize)]
    struct NativeChildDenialEntry<'a> {
        event: &'static str,
        policy: &'static str,
        root_cmd: &'a str,
        root_args: &'a [String],
        child_pid: u32,
        child_name: &'a str,
        child_image: &'a str,
        matched_by: &'a str,
        matched_operation: &'a str,
        cwd: String,
        scope: &'static str,
        ts: u64,
    }

    append_serialized(
        path,
        &NativeChildDenialEntry {
            event: "native_dependency_mutation_denied",
            policy: "native-deny",
            root_cmd: denial.root_cmd,
            root_args: denial.root_args,
            child_pid: denial.child_pid,
            child_name: denial.child_name,
            child_image: denial.child_image,
            matched_by: denial.matched_by,
            matched_operation: denial.matched_operation,
            cwd: denial.cwd.display().to_string(),
            scope: "root_process_tree",
            ts: now_ts(),
        },
    )
}

fn append_serialized(path: &Path, value: &impl Serialize) -> anyhow::Result<()> {
    let _guard = APPEND_LOCK
        .lock()
        .map_err(|_| anyhow::anyhow!("audit append lock was poisoned"))?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut line = serde_json::to_vec(value)?;
    line.push(b'\n');
    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    f.write_all(&line)?;
    Ok(())
}

/// Timestamp hiện tại (giây kể từ epoch).
pub fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
