use serde::Serialize;

use super::super::append_serialized;
use crate::audit::test::support::temp_dir;

#[derive(Serialize)]
struct Record {
    event: &'static str,
    index: usize,
}

#[test]
fn creates_parent_and_appends_one_json_record_per_line() {
    let dir = temp_dir();
    let path = dir.path().join("nested/audit.jsonl");
    append_serialized(
        &path,
        &Record {
            event: "check",
            index: 1,
        },
    )
    .unwrap();
    append_serialized(
        &path,
        &Record {
            event: "check",
            index: 2,
        },
    )
    .unwrap();

    let contents = std::fs::read_to_string(path).unwrap();
    let rows = contents.lines().collect::<Vec<_>>();
    assert_eq!(rows.len(), 2);
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(rows[0]).unwrap()["index"],
        1
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(rows[1]).unwrap()["index"],
        2
    );
}
