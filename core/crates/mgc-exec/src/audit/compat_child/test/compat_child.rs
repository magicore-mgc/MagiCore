use super::super::append_compat_child;
use crate::audit::test::support::temp_dir;

#[test]
fn writes_the_observed_child_event_schema() {
    let dir = temp_dir();
    let path = dir.path().join("exec.jsonl");
    let args = vec!["install".to_string(), "[REDACTED]".to_string()];
    append_compat_child(&path, "npm", 42, "/bin/npm", "npm", &args, dir.path()).unwrap();

    let line = std::fs::read_to_string(path).unwrap();
    let row: serde_json::Value = serde_json::from_str(line.trim()).unwrap();
    assert_eq!(row["event"], "compat_runner_child_observed");
    assert_eq!(row["manager"], "npm");
    assert_eq!(row["pid"], 42);
    assert_eq!(row["image"], "/bin/npm");
    assert_eq!(row["program"], "npm");
    assert_eq!(row["args"][1], "[REDACTED]");
    assert_eq!(row["scope"], "root_process_tree");
    assert!(row["ts"].as_u64().is_some());
}
