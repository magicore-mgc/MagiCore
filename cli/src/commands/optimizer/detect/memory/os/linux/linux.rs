//! Linux cgroup RAM discovery and public fixture parser.
//! (Đọc giới hạn RAM cgroup Linux và cung cấp parser fixture.)
#[cfg(any(target_os = "linux", test))]
#[cfg(any(target_os = "linux", test))]
use std::path::Path;

/// Prefer the kernel cgroup limit when present, otherwise use host RAM.
/// (Ưu tiên giới hạn cgroup của kernel; nếu không có thì dùng RAM host.)
#[cfg(target_os = "linux")]
// `memory.rs` is two levels above this module, so the reader must be reachable
// there; `pub(super)` stopped at `os` and failed on Linux with E0603.
// (`memory.rs` cao hơn module này hai mức nên hàm đọc phải tới được đó;
// `pub(super)` dừng ở `os` và fail trên Linux với E0603.)
pub(in crate::commands::optimizer::detect::memory) fn linux_cgroup_memory_limit_bytes()
-> Option<u64> {
    let memberships = std::fs::read_to_string("/proc/self/cgroup").unwrap_or_default();
    HardwareInfo::linux_cgroup_memory_limit_bytes_at(Path::new("/sys/fs/cgroup"), &memberships)
}

impl HardwareInfo {
    /// Read every finite process-to-root limit and use the minimum: a child
    /// cgroup may say unlimited while one of its parents is capped.
    /// (Đọc mọi giới hạn từ cgroup process tới root và lấy min; cgroup cha có
    /// thể bị giới hạn dù cgroup con ghi `max`.)
    #[cfg(any(target_os = "linux", test))]
    pub(crate) fn linux_cgroup_memory_limit_bytes_at(
        root: &Path,
        memberships: &str,
    ) -> Option<u64> {
        let mut paths = Vec::new();
        for line in memberships.lines() {
            let mut fields = line.splitn(3, ':');
            let Some(hierarchy) = fields.next() else {
                continue;
            };
            let Some(controllers) = fields.next() else {
                continue;
            };
            let Some(group) = fields.next() else {
                continue;
            };
            let Ok(relative) = Path::new(group).strip_prefix("/") else {
                continue;
            };
            if relative
                .components()
                .any(|part| !matches!(part, std::path::Component::Normal(_)))
            {
                continue;
            }
            if hierarchy == "0" && controllers.is_empty() {
                for ancestor in relative.ancestors() {
                    paths.push(root.join(ancestor).join("memory.max"));
                }
            } else if controllers
                .split(',')
                .any(|controller| controller == "memory")
            {
                for ancestor in relative.ancestors() {
                    paths.push(
                        root.join("memory")
                            .join(ancestor)
                            .join("memory.limit_in_bytes"),
                    );
                }
            }
        }
        paths.sort();
        paths.dedup();
        paths
            .into_iter()
            .filter_map(|path| std::fs::read_to_string(path).ok())
            .filter_map(|text| Self::parse_cgroup_memory_limit_bytes(&text))
            .min()
    }
}
use crate::commands::optimizer::detect::HardwareInfo;
