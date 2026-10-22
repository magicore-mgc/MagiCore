// Audit command planning tests — verify the signature flag boundary.
// Test plan audit command — xác nhận ranh giới flag audit chữ ký.
use super::{AuditPlan, plan};
use crate::commands::definitions::AuditCmd;

#[test]
fn signature_plan_preserves_identity_and_token_fields() {
    let actual = plan(
        Some(AuditCmd::Signatures {
            package: "sample".into(),
            protocol: "npm".into(),
            registry: "private".into(),
            token: Some("test-token".into()),
            json: true,
        }),
        false,
        None,
    )
    .unwrap();

    assert_eq!(
        actual,
        AuditPlan::Signatures {
            package: "sample".into(),
            protocol: "npm".into(),
            registry: "private".into(),
            token: Some("test-token".into()),
            json: true,
        }
    );
}

#[test]
fn signature_plan_rejects_fix_flag_with_existing_error() {
    let actual = plan(
        Some(AuditCmd::Signatures {
            package: "sample".into(),
            protocol: "npm".into(),
            registry: "private".into(),
            token: None,
            json: false,
        }),
        true,
        None,
    )
    .unwrap_err();

    assert_eq!(
        actual.to_string(),
        crate::error::audit_signatures_flags_conflict().to_string()
    );
}

#[test]
fn signature_plan_rejects_format_flag_with_existing_error() {
    let actual = plan(
        Some(AuditCmd::Signatures {
            package: "sample".into(),
            protocol: "npm".into(),
            registry: "private".into(),
            token: None,
            json: false,
        }),
        false,
        Some("json".into()),
    )
    .unwrap_err();

    assert_eq!(
        actual.to_string(),
        crate::error::audit_signatures_flags_conflict().to_string()
    );
}

#[test]
fn package_audit_plan_keeps_fix_and_format() {
    assert_eq!(
        plan(None, true, Some("json".into())).unwrap(),
        AuditPlan::Package {
            fix: true,
            format: Some("json".into()),
        }
    );
}
