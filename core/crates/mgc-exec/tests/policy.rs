//! Policy resolver tests — kiểm thử bộ phân giải policy.
//!
//! Covers the runtime rows of the dev-exec decision table (Spec rev4 §7):
//! mutation-deny precedence, exact exceptions, profile scope, compat route.
//! (Bao phủ các hàng runtime của decision table dev-exec (Spec rev4 §7):
//! ưu tiên mutation-deny, exception chính xác, phạm vi profile, đường compat.)

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use mgc_exec::policy::{
    DenyCode, Invocation, LaneProfile, PolicyDecision, PolicyMode, ReadOnlyException,
    invocation_key_for_tests, is_mutation_invocation, missing_exception, resolve,
};

fn inv(program: &str, args: &[&str], in_compat_subtree: bool) -> Invocation {
    Invocation {
        program: program.to_string(),
        args: args.iter().map(|a| a.to_string()).collect(),
        env: Vec::new(),
        in_compat_subtree,
    }
}

fn lane() -> LaneProfile {
    LaneProfile {
        framework: "next".to_string(),
        version: "16.3.8".to_string(),
        os: "darwin".to_string(),
        operation: "dev".to_string(),
        allowed_pm_invocations: vec!["npm --version".to_string()],
        requires_pm: vec![],
    }
}

fn exc() -> ReadOnlyException {
    ReadOnlyException {
        program: "npm".to_string(),
        args_exact: vec!["--version".to_string()],
        unexpired: true,
        answer: None,
    }
}

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

// Only the closed version-query forms are eligible for signed exceptions.
// Chỉ dạng truy vấn version trong danh sách đóng mới đủ điều kiện exception.
#[test]
fn exception_only_accepts_closed_read_only_probes() {
    use mgc_exec::policy::is_read_only_probe;

    for args in [vec!["--version".to_string()], vec!["-v".to_string()]] {
        assert!(is_read_only_probe("npm", &args));
    }
    for args in [
        vec!["exec".to_string(), "x".to_string()],
        vec!["run".to_string(), "dev".to_string()],
        vec!["view".to_string(), "x".to_string()],
        vec!["install".to_string(), "x".to_string()],
    ] {
        assert!(!is_read_only_probe("npm", &args));
    }
    assert!(!is_read_only_probe("fakepm", &["--version".to_string()]));
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

// Verb tables: representative mutation probes per ecosystem.
// (Bảng động từ: probe mutation đại diện mỗi ecosystem.)
#[test]
fn mutation_verb_tables() {
    let cases = [
        ("pip", vec!["install", "x"], true),
        ("uv", vec!["sync"], true),
        ("cargo", vec!["add", "x"], true),
        ("cargo", vec!["build", "--locked", "--offline"], false),
        ("cargo", vec!["build"], true),
        ("pub", vec!["get"], true),
        ("composer", vec!["require", "x"], true),
        ("go", vec!["get", "x"], true),
        ("go", vec!["mod", "download"], true),
        ("go", vec!["mod", "verify"], true),
        ("deno", vec!["publish"], true),
        ("npm", vec!["--version"], false),
        ("npm", vec![], false),
        ("node", vec!["server.js"], false),
        ("zig", vec!["run", "probe.zig"], false),
        ("zig", vec!["version"], false),
        ("zig", vec!["fetch", "https://example.invalid/pkg"], true),
        ("zig", vec!["build"], true),
        ("zig", vec!["run", "probe.zig", "--fetch"], true),
    ];
    for (program, args, expected) in cases {
        let owned: Vec<String> = args.iter().map(|a| a.to_string()).collect();
        assert_eq!(
            is_mutation_invocation(program, &owned, &[]),
            expected,
            "program={program} args={args:?}"
        );
    }
}

// Missing-exception helper reports the row-5 code.
// (Helper thiếu exception báo mã hàng 5.)
#[test]
fn missing_exception_helper() {
    assert_eq!(
        missing_exception(),
        PolicyDecision::Deny {
            code: DenyCode::MissingException
        }
    );
}

// Profile key helper is stable for comparisons.
// (Helper key profile ổn định để so sánh.)
#[test]
fn profile_key_format() {
    assert_eq!(
        invocation_key_for_tests("npm", &["--version".to_string()]),
        "npm --version"
    );
    assert_eq!(invocation_key_for_tests("/usr/bin/npm", &[]), "npm");
}

// MGC-synthesized probes: exactly one today (`npm --version`).
// (Probe do MGC tự trả lời: hiện đúng một (`npm --version`).)
#[test]
fn self_answer_table() {
    use mgc_exec::policy::self_answer_for;
    assert_eq!(
        self_answer_for("npm", &["--version".to_string()]),
        Some("0.0.0".to_string())
    );
    assert_eq!(self_answer_for("npm", &["install".to_string()]), None);
    assert_eq!(self_answer_for("node", &["--version".to_string()]), None);
}
