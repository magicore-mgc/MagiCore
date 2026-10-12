//! Spawn-profile store — lưu trữ spawn-profile.
//!
//! A spawn-profile is evidence, not a permission slip (Spec rev4 §6/§11):
//! it records which PM invocations were *observed* for one framework,
//! version, OS and operation tuple. At runtime it operates as a
//! default-deny policy for profiled lanes; lanes without a profile get
//! tripwire-only treatment (see [`crate::policy::resolve`]).
//! (Spawn-profile là bằng chứng, không phải giấy phép (Spec rev4 §6/§11):
//! nó ghi invocation PM nào *đã quan sát* cho một bộ framework, version, OS
//! và operation. Lúc runtime nó vận hành như policy default-deny cho lane đã
//! profile; lane chưa profile chỉ có tripwire (xem [`crate::policy::resolve`]).)
//!
//! Profiles expire when the lock they were captured against changes: a new
//! dependency set may spawn new children.
//! (Profile hết hạn khi lock lúc capture đổi: tập dependency mới có thể đẻ
//! child mới.)

use serde::{Deserialize, Serialize};

use crate::policy::LaneProfile;

/// Store schema version — phiên bản schema store.
pub const SPAWN_PROFILE_SCHEMA: &str = "spawn-profile-v1";

/// Stored profile with capture metadata — profile lưu kèm metadata capture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnProfileFile {
    /// Schema version — must equal [`SPAWN_PROFILE_SCHEMA`].
    /// Phiên bản schema — phải bằng [`SPAWN_PROFILE_SCHEMA`].
    pub schema_version: String,
    /// MagiCore version that captured this profile.
    /// Version MagiCore đã capture profile này.
    pub mgc_version: String,
    /// Corpus snapshot id the capture belongs to (empty = ad-hoc).
    /// Id snapshot corpus của lần capture (rỗng = ad-hoc).
    #[serde(default)]
    pub corpus_id: String,
    /// Digest of the `mgc.lock` the capture ran against; a different lock
    /// means re-verification (Spec rev4 §6).
    /// Digest của `mgc.lock` lúc capture; lock khác nghĩa là re-verify
    /// (Spec rev4 §6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_digest: Option<String>,
    /// Unix seconds when captured — giây unix lúc capture.
    pub captured_at: i64,
    /// The observed lane profile — profile lane đã quan sát.
    pub profile: LaneProfile,
}

/// Profile lookup failure — lỗi tra cứu profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileLookupError {
    /// No profile file for this tuple — không có file profile cho bộ này.
    NotFound,
    /// Profile file failed to parse or has an unknown schema.
    /// File profile parse hỏng hoặc schema lạ.
    Invalid(String),
}

/// File name for one tuple — tên file cho một bộ.
pub fn profile_file_name(framework: &str, operation: &str, os: &str) -> String {
    let safe = |value: &str| {
        value
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '-'
                }
            })
            .collect::<String>()
    };
    format!("{}-{}-{}.json", safe(framework), safe(operation), safe(os))
}

/// Load the profile for an exact tuple and require its lock digest to match.
/// Returns `None` when no profile exists (lane stays level-2 tripwire-only).
/// (Nạp profile cho đúng bộ và yêu cầu khớp lock digest. Trả `None` khi không
/// có profile (lane ở mức 2 tripwire-only).)
pub fn find_active_profile(
    store_dir: &std::path::Path,
    framework: &str,
    framework_version: &str,
    os: &str,
    operation: &str,
    lock_digest: Option<&str>,
) -> Result<Option<LaneProfile>, ProfileLookupError> {
    let path = store_dir.join(profile_file_name(framework, operation, os));
    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok(None);
        }
        Err(e) => return Err(ProfileLookupError::Invalid(e.to_string())),
    };
    let file: SpawnProfileFile =
        serde_json::from_slice(&bytes).map_err(|e| ProfileLookupError::Invalid(e.to_string()))?;
    if file.schema_version != SPAWN_PROFILE_SCHEMA {
        return Err(ProfileLookupError::Invalid(format!(
            "unknown schema {}",
            file.schema_version
        )));
    }
    // Exact version pin: a range match would need per-range evidence.
    // (Ghim version chính xác: khớp range cần bằng chứng theo range.)
    if file.profile.framework != framework
        || file.profile.version != framework_version
        || file.profile.os != os
        || file.profile.operation != operation
    {
        return Ok(None);
    }
    // Lock drift expires the profile (Spec rev4 §6).
    // (Lock trôi làm profile hết hạn (Spec rev4 §6).)
    if file.lock_digest.as_deref() != lock_digest {
        return Ok(None);
    }
    Ok(Some(file.profile))
}

/// Store a profile file (creates parent dirs).
/// Lưu file profile (tạo parent dirs).
pub fn store_profile(store_dir: &std::path::Path, file: &SpawnProfileFile) -> anyhow::Result<()> {
    std::fs::create_dir_all(store_dir)?;
    let path = store_dir.join(profile_file_name(
        &file.profile.framework,
        &file.profile.operation,
        &file.profile.os,
    ));
    let text = serde_json::to_string_pretty(file)?;
    std::fs::write(path, text)?;
    Ok(())
}

/// Record every distinct `(basename, args)` pair in the live subtree of
/// `root_pid` for `duration`, sampling every `interval`.
/// Returns the sorted, deduplicated observation set — the raw material for
/// a spawn-profile (PM filtering happens at policy time, not here, so the
/// record stays a faithful observation).
/// (Ghi mọi cặp `(basename, args)` phân biệt trong subtree sống của
/// `root_pid` trong `duration`, lấy mẫu mỗi `interval`. Trả tập quan sát đã
/// sắp xếp, khử trùng — nguyên liệu thô cho spawn-profile (lọc PM ở lúc áp
/// policy, không ở đây, để bản ghi trung thực.)
pub fn record_subtree(
    root_pid: u32,
    duration: std::time::Duration,
    interval: std::time::Duration,
) -> Vec<(String, Vec<String>)> {
    use std::collections::{HashMap, HashSet};
    use sysinfo::{Pid, ProcessesToUpdate, System};

    let mut seen: HashSet<(String, Vec<String>)> = HashSet::new();
    let deadline = std::time::Instant::now() + duration;
    let mut system = System::new();
    while std::time::Instant::now() < deadline {
        system.refresh_processes(ProcessesToUpdate::All, true);
        // Parent map first so short-lived children still resolve.
        // (Map parent trước để child ngắn ngủi vẫn resolve được.)
        let mut parent_of: HashMap<Pid, Pid> = HashMap::new();
        for (pid, process) in system.processes() {
            if let Some(parent) = process.parent() {
                parent_of.insert(*pid, parent);
            }
        }
        let mut in_subtree = HashSet::from([Pid::from(root_pid as usize)]);
        let mut grown = true;
        while grown {
            grown = false;
            for (pid, parent) in &parent_of {
                if in_subtree.contains(parent) && in_subtree.insert(*pid) {
                    grown = true;
                }
            }
        }
        for pid in &in_subtree {
            if let Some(process) = system.process(*pid) {
                let name = process.name().to_string_lossy().to_string();
                let args: Vec<String> = process
                    .cmd()
                    .iter()
                    .skip(1)
                    .map(|s| s.to_string_lossy().to_string())
                    .collect();
                seen.insert((name, args));
            }
        }
        std::thread::sleep(interval);
    }
    let mut out: Vec<(String, Vec<String>)> = seen.into_iter().collect();
    out.sort();
    out
}
