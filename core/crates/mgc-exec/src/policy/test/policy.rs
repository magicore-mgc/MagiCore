use crate::policy::test::support::{exc, inv, lane};
use crate::policy::{DenyCode, PolicyDecision, PolicyMode, is_mutation_invocation, resolve};

// Mutation verbs are denied in native mode, no matter the lane.
// (Động từ mutation bị chặn ở native mode, bất kể lane.)
#[test]
fn native_npm_install_is_denied() {
    assert!(is_mutation_invocation("npm", &["install".to_string()], &[]));
    assert_eq!(
        resolve(
            &PolicyMode::Native,
            &inv("npm", &["install"], false),
            None,
            None
        ),
        PolicyDecision::Deny {
            code: DenyCode::NativeMutationDeny
        }
    );
}

// A profile listing a mutation can never turn it into allow.
// (Profile liệt kê mutation cũng không biến nó thành cho phép.)
#[test]
fn profile_cannot_allow_mutation() {
    let mut prof = lane();
    prof.allowed_pm_invocations.push("npm install".to_string());
    assert_eq!(
        resolve(
            &PolicyMode::Native,
            &inv("npm", &["install"], false),
            Some(&prof),
            None
        ),
        PolicyDecision::Deny {
            code: DenyCode::NativeMutationDeny
        }
    );
}

// Exact read-only exception inside the active profile is allowed.
// (Exception đọc chính xác trong profile hiệu lực được cho phép.)
#[test]
fn exact_exception_in_profile_is_allowed() {
    assert_eq!(
        resolve(
            &PolicyMode::Native,
            &inv("npm", &["--version"], false),
            Some(&lane()),
            Some(&exc())
        ),
        PolicyDecision::Allow
    );
}

// No active profile can authorize a spawned exception, even if the signed
// manifest matches exactly. Unprofiled lanes may only use MGC self-answers.
// Không profile hiệu lực thì manifest không thể cho phép spawn exception.
#[test]
fn exception_without_profile_is_denied() {
    assert_eq!(
        resolve(
            &PolicyMode::Native,
            &inv("npm", &["--version"], false),
            None,
            Some(&exc())
        ),
        PolicyDecision::Deny {
            code: DenyCode::ForbiddenNameNoException
        }
    );
}

// MGC-synthesized answers spawn nothing.
// (Câu trả lời do MGC tổng hợp thì không spawn gì.)
#[test]
fn self_answer_returns_payload() {
    let mut e = exc();
    e.answer = Some("0.0.0".to_string());
    assert_eq!(
        resolve(
            &PolicyMode::Native,
            &inv("npm", &["--version"], false),
            Some(&lane()),
            Some(&e)
        ),
        PolicyDecision::SelfAnswer("0.0.0".to_string())
    );
}

// Expired exception does not open the gate (row-5 code, not row-6:
// the invocation is in-profile, but the exception path failed).
// (Exception hết hạn không mở cổng (mã hàng 5, không phải hàng 6:
// invocation trong profile, nhưng đường exception hỏng).)
#[test]
fn expired_exception_is_denied() {
    let mut e = exc();
    e.unexpired = false;
    assert_eq!(
        resolve(
            &PolicyMode::Native,
            &inv("npm", &["--version"], false),
            Some(&lane()),
            Some(&e)
        ),
        PolicyDecision::Deny {
            code: DenyCode::MissingException
        }
    );
}

// Profiled lane, PM invocation outside the profile → deny.
// (Lane đã profile, invocation PM ngoài profile → deny.)
#[test]
fn profiled_lane_outside_profile_is_denied() {
    assert_eq!(
        resolve(
            &PolicyMode::Native,
            &inv("npm", &["ls"], false),
            Some(&lane()),
            None
        ),
        PolicyDecision::Deny {
            code: DenyCode::OutsideActiveProfile
        }
    );
}

// Unprofiled lane, forbidden name without exception → deny.
// (Lane chưa profile, tên cấm không exception → deny.)
#[test]
fn unprofiled_lane_forbidden_name_is_denied() {
    assert_eq!(
        resolve(
            &PolicyMode::Native,
            &inv("pip", &["install", "x"], false),
            None,
            None
        ),
        PolicyDecision::Deny {
            code: DenyCode::NativeMutationDeny
        }
    );
    assert_eq!(
        resolve(&PolicyMode::Native, &inv("npm", &["ls"], false), None, None),
        PolicyDecision::Deny {
            code: DenyCode::ForbiddenNameNoException
        }
    );
}

// Ordinary descendants run with observability audit (not a safety proof).
// (Descendant thường chạy kèm audit quan sát (không phải chứng minh an toàn).)
#[test]
fn ordinary_descendant_is_allowed() {
    assert_eq!(
        resolve(
            &PolicyMode::Native,
            &inv("node", &["server.js"], false),
            None,
            None
        ),
        PolicyDecision::Allow
    );
    assert_eq!(
        resolve(
            &PolicyMode::Native,
            &inv("node", &["server.js"], false),
            Some(&lane()),
            None
        ),
        PolicyDecision::Allow
    );
}

// Compat subtree: work runs with warning + audit, including mutation.
// (Subtree compat: việc chạy kèm cảnh báo + audit, kể cả mutation.)
#[test]
fn compat_subtree_yields_compat_audit() {
    let mode = PolicyMode::CompatRunner {
        manager: "npm".to_string(),
    };
    assert!(matches!(
        resolve(&mode, &inv("npm", &["install"], true), None, None),
        PolicyDecision::CompatAudit { .. }
    ));
    assert!(matches!(
        resolve(&mode, &inv("node", &["server.js"], true), None, None),
        PolicyDecision::CompatAudit { .. }
    ));
}

// Outside the declared subtree, compat grants nothing.
// (Ngoài subtree đã khai báo, compat không cấp gì.)
#[test]
fn outside_compat_subtree_is_denied() {
    let mode = PolicyMode::CompatRunner {
        manager: "npm".to_string(),
    };
    assert_eq!(
        resolve(&mode, &inv("npm", &["install"], false), None, None),
        PolicyDecision::Deny {
            code: DenyCode::NativeMutationDeny
        }
    );
}
