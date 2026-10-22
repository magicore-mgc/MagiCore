// MGC-synthesized probes: exactly one today (`npm --version`).
// (Probe do MGC tự trả lời: hiện đúng một (`npm --version`).)
#[test]
fn self_answer_table() {
    use crate::policy::self_answer_for;
    assert_eq!(
        self_answer_for("npm", &["--version".to_string()]),
        Some("0.0.0".to_string())
    );
    assert_eq!(self_answer_for("npm", &["install".to_string()]), None);
    assert_eq!(self_answer_for("node", &["--version".to_string()]), None);
}
