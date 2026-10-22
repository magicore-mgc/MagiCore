// Tests that audit contexts retain their configured project scope.
// Test context audit giữ đúng phạm vi project được cấu hình.

use super::*;

#[test]
fn compat_and_native_contexts_keep_paths_and_invocation_evidence() {
    let root = Path::new("project-root");
    let log = Path::new("audit.log");
    let args = vec!["build".to_owned(), "--release".to_owned()];
    let compat = CompatAuditContext {
        manager: "npm",
        log_path: log,
        cwd: root,
        seen_processes: Mutex::new(std::collections::HashSet::new()),
    };
    let native = NativeAuditContext {
        log_path: log,
        root_cmd: "mgc build",
        root_args: &args,
        cwd: root,
    };

    assert_eq!(compat.manager, "npm");
    assert_eq!(compat.log_path, log);
    assert!(compat.seen_processes.lock().expect("dedup state must be healthy").is_empty());
    assert_eq!(native.root_cmd, "mgc build");
    assert_eq!(native.root_args, args);
    assert_eq!(native.cwd, root);
}
