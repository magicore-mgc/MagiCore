//! Unsupported journal schema rejection contract.
//! Contract từ chối schema journal không được hỗ trợ.

use super::super::parse_mutation_journal;

#[test]
fn v1_journal_schema_fails_closed_explicitly() {
    // P1-3: schema v1 (chưa từng public — RC branch only) bị từ chối rõ
    // ràng, không migrate thầm, không đoán.
    // (v1 journals fail with an explicit schema error.)
    let raw = concat!(
        r#"{"v":1,"op":"remove","pid":1,"packages":[],"lock_existed":false,"pre_digest":[],"post_digest":null,"#,
        r#""state":"in_progress"}"#,
    );
    let err = parse_mutation_journal(raw, std::path::Path::new("journal.json")).unwrap_err();
    assert!(
        format!("{err:#}").contains("unsupported mutation journal schema v1"),
        "must name the unsupported schema: {err:#}"
    );
}
