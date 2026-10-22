use crate::policy::PolicyDecision;

#[test]
fn decision_variants_keep_distinct_payloads() {
    assert_ne!(
        PolicyDecision::Allow,
        PolicyDecision::SelfAnswer("0.0.0".to_string())
    );
    assert_eq!(
        PolicyDecision::SelfAnswer("0.0.0".to_string()),
        PolicyDecision::SelfAnswer("0.0.0".to_string())
    );
    assert_eq!(
        PolicyDecision::CompatAudit {
            note: "audit".to_string()
        },
        PolicyDecision::CompatAudit {
            note: "audit".to_string()
        }
    );
}
