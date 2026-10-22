//! Serialized prune report fields remain stable for scripts and agents.
//! Các field JSON của báo cáo prune phải ổn định cho script và agent.

use crate::commands::store::PruneReport;

#[test]
fn report_keeps_existing_json_field_names_and_types() {
    let report = PruneReport {
        unreferenced: vec!["react@18.2.0".to_owned()],
        removed: vec!["react@18.2.0".to_owned()],
        removed_bytes: 7,
        dry_run: true,
    };

    let json = serde_json::to_value(report).unwrap();
    let object = json.as_object().unwrap();

    assert_eq!(object.len(), 4);
    assert_eq!(json["unreferenced"][0], "react@18.2.0");
    assert_eq!(json["removed"][0], "react@18.2.0");
    assert_eq!(json["removed_bytes"], 7);
    assert_eq!(json["dry_run"], true);
}
