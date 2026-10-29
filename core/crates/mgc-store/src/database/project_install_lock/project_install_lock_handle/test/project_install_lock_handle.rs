#![allow(clippy::unwrap_used)]

use super::super::ProjectInstallLock;

/// SEC-07: every spelling of one project must map to the same lock file, so a
/// second install cannot enter through a different path string.
/// (SEC-07: mọi cách viết của cùng một project phải ra cùng một file lock để
/// lần cài thứ hai không lọt qua chuỗi path khác.)
#[cfg(unix)]
#[test]
fn path_spellings_of_one_project_share_a_single_lock() {
    let root = tempfile::tempdir().unwrap();
    let locks = root.path().join("locks");
    let project = root.path().join("project");
    std::fs::create_dir_all(&project).unwrap();

    let first = ProjectInstallLock::acquire_at(&locks, project.to_str().unwrap()).unwrap();
    let first_path = first.path().to_path_buf();

    // `.` inside the path is the same directory.
    // (`.` trong path là cùng thư mục.)
    let dotted = format!("{}/.", project.display());
    let same = ProjectInstallLock::acquire_at(&locks, &dotted);
    // The OS lock is held, so both spellings must resolve to the SAME lock
    // file: the second attempt is refused by contention, not by a new file.
    // (OS lock đang giữ nên cả hai cách viết phải ra CÙNG file lock: lần hai bị
    // từ chối do tranh lock, không phải do file mới.)
    match same {
        Ok(lock) => {
            // A same-process flock is re-entrant on some platforms; the path
            // equality below is the assertion that matters.
            // (flock cùng process có thể tái nhập trên vài nền tảng; đẳng thức
            // path bên dưới mới là điều cần khẳng định.)
            assert_eq!(lock.path(), first_path.as_path());
        }
        Err(_) => {
            assert_eq!(
                std::fs::read_dir(&locks).unwrap().count(),
                1,
                "a different spelling must not create a second lock file"
            );
        }
    }
}

/// Two genuinely different projects still get separate locks.
/// (Hai project thật sự khác nhau vẫn có lock riêng.)
#[test]
fn different_projects_get_distinct_locks() {
    let root = tempfile::tempdir().unwrap();
    let locks = root.path().join("locks");
    let a = root.path().join("a");
    let b = root.path().join("b");
    std::fs::create_dir_all(&a).unwrap();
    std::fs::create_dir_all(&b).unwrap();

    let _first = ProjectInstallLock::acquire_at(&locks, a.to_str().unwrap()).unwrap();
    let _second = ProjectInstallLock::acquire_at(&locks, b.to_str().unwrap()).unwrap();
    assert_eq!(std::fs::read_dir(&locks).unwrap().count(), 2);
}
