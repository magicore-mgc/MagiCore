//! Nearest-owner hardware detection tests.
//! (Kiểm thử phát hiện phần cứng đặt cạnh owner bộ nhớ Linux.)

use crate::commands::optimizer::detect::HardwareInfo;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_linux_cgroup_memory_limit_reads_nested_process_membership() {
    use std::path::Path;

    let dir = tempdir().unwrap();
    let cgroup_root = dir.path().join("sys/fs/cgroup");
    let nested = cgroup_root.join("system.slice/mgc.scope");
    fs::create_dir_all(&nested).unwrap();
    fs::write(nested.join("memory.max"), "4294967296\n").unwrap();

    assert_eq!(
        HardwareInfo::linux_cgroup_memory_limit_bytes_at(
            Path::new(&cgroup_root),
            "0::/system.slice/mgc.scope\n"
        ),
        Some(4294967296)
    );
    assert_eq!(
        HardwareInfo::linux_cgroup_memory_limit_bytes_at(
            Path::new(&cgroup_root),
            "0::/../../outside\n"
        ),
        None
    );
}

#[test]
fn test_linux_cgroup_memory_limit_uses_effective_ancestor_limit() {
    use std::path::Path;

    let dir = tempdir().unwrap();
    let cgroup_root = dir.path().join("sys/fs/cgroup");
    let parent = cgroup_root.join("tenant");
    let nested = parent.join("worker");
    fs::create_dir_all(&nested).unwrap();
    fs::write(parent.join("memory.max"), "8589934592\n").unwrap();
    fs::write(nested.join("memory.max"), "max\n").unwrap();
    fs::write(cgroup_root.join("memory.max"), "17179869184\n").unwrap();

    assert_eq!(
        HardwareInfo::linux_cgroup_memory_limit_bytes_at(
            Path::new(&cgroup_root),
            "0::/tenant/worker\n"
        ),
        Some(8589934592)
    );
}
