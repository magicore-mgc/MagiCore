use super::super::AuditEntry;

#[test]
fn preserves_the_json_field_names_and_values() {
    let entry = AuditEntry {
        cmd: "cargo".to_string(),
        args: vec!["[REDACTED]".to_string()],
        cwd: "/project".to_string(),
        exit_code: 0,
        duration_ms: 15,
        dry_run: false,
        ts: 1_800_000_000,
    };
    let json = serde_json::to_value(entry).unwrap();
    assert_eq!(json["cmd"], "cargo");
    assert_eq!(json["args"][0], "[REDACTED]");
    assert_eq!(json["cwd"], "/project");
    assert_eq!(json["exit_code"], 0);
    assert_eq!(json["duration_ms"], 15);
    assert_eq!(json["dry_run"].as_bool(), Some(false));
    assert_eq!(json["ts"], 1_800_000_000_u64);
}
