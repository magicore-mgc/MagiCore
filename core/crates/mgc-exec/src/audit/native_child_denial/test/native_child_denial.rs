use std::path::Path;

use super::super::{NativeChildDenial, append_native_child_denial};
use crate::audit::test::support::temp_dir;

#[test]
fn writes_the_native_deny_event_without_unredacted_root_args() {
    let dir = temp_dir();
    let path = dir.path().join("exec.jsonl");
    let root_args = vec!["[REDACTED]".to_string()];
    append_native_child_denial(
        &path,
        NativeChildDenial {
            root_cmd: "node",
            root_args: &root_args,
            cwd: Path::new("/project"),
            child_pid: 52,
            child_name: "cargo",
            child_image: "/usr/bin/cargo",
            matched_by: "program",
            matched_operation: "fetch",
        },
    )
    .unwrap();

    let line = std::fs::read_to_string(path).unwrap();
    let row: serde_json::Value = serde_json::from_str(line.trim()).unwrap();
    assert_eq!(row["event"], "native_dependency_mutation_denied");
    assert_eq!(row["policy"], "native-deny");
    assert_eq!(row["root_args"][0], "[REDACTED]");
    assert_eq!(row["child_name"], "cargo");
    assert_eq!(row["matched_by"], "program");
    assert_eq!(row["matched_operation"], "fetch");
    assert_eq!(row["scope"], "root_process_tree");
}
