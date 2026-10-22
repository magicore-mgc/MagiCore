use super::super::append_compat_scope;
use crate::audit::test::support::temp_dir;

#[test]
fn writes_the_declared_scope_event_schema() {
    let dir = temp_dir();
    let path = dir.path().join("exec.jsonl");
    let args = vec!["install".to_string(), "[REDACTED]".to_string()];
    append_compat_scope(&path, "npm", "/bin/npm", &args, dir.path(), false).unwrap();

    let line = std::fs::read_to_string(path).unwrap();
    let row: serde_json::Value = serde_json::from_str(line.trim()).unwrap();
    assert_eq!(row["event"], "compat_runner_scope");
    assert_eq!(row["manager"], "npm");
    assert_eq!(row["root_cmd"], "/bin/npm");
    assert_eq!(row["root_args"][1], "[REDACTED]");
    assert_eq!(row["scope"], "root_process_tree");
    assert_eq!(row["dry_run"], false);
    assert!(row["ts"].as_u64().is_some());
}
