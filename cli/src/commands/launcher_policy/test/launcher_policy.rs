use super::super::{LauncherPolicy, Runtime};

#[test]
fn test_policy_constructors_default_to_disallowing_dangerous_permissions() {
    for runtime in [Runtime::Bun, Runtime::Deno, Runtime::Node] {
        assert!(!LauncherPolicy::dev_server(runtime).allow_dangerous_permissions);
        assert!(!LauncherPolicy::test_runner(runtime).allow_dangerous_permissions);
    }
}

// ── Node and tsx must be policed, not just Bun/Deno ─────────────────────
// The dev launcher validated Bun and Deno while `node` and `tsx` ran with no
// check at all, so `node -e "<code>"` in a `mgc.toml` script executed arbitrary
// code through the launcher that owns the flag policy.
// (Launcher dev kiểm Bun/Deno nhưng `node`/`tsx` chạy không kiểm gì, nên
// `node -e "<code>"` trong script `mgc.toml` chạy mã tùy ý.)

use super::*;

#[test]
fn node_inline_eval_is_refused() {
    for args in [
        vec!["-e", "require('child_process').exec('id')"],
        vec!["--eval", "process.exit(0)"],
        vec!["-p", "process.env"],
        vec!["--print", "1"],
    ] {
        let error = LauncherPolicy::dev_server(Runtime::Node)
            .validate_args(&args)
            .expect_err(&format!("node {args:?} must be refused"));
        // The policy matches in list order, so `--print` may be reported as `-p`;
        // what matters is that the refusal names a dangerous flag and the runtime.
        // (Policy khớp theo thứ tự list nên `--print` có thể báo là `-p`; điều
        // quan trọng là lỗi nêu một flag nguy hiểm và runtime.)
        let message = error.to_string();
        assert!(message.contains("dangerous node flag"), "{message}");
        assert!(message.contains("project scripts only"), "{message}");
    }
}

/// `tsx` forwards flags to Node, so the same inline-eval spellings must fail.
/// (`tsx` chuyển flag cho Node nên các cách viết inline-eval cũng phải fail.)
#[test]
fn tsx_inline_eval_is_refused_because_it_forwards_to_node() {
    for args in [vec!["-e", "code"], vec!["--eval", "code"]] {
        LauncherPolicy::dev_server(Runtime::Tsx)
            .validate_args(&args)
            .expect_err(&format!("tsx {args:?} must be refused"));
    }
}

/// A plain script path stays allowed for both runtimes: the policy blocks the
/// flag, not the runtime.
/// (Đường dẫn script bình thường vẫn được phép — policy chặn flag, không chặn runtime.)
#[test]
fn ordinary_script_paths_stay_allowed() {
    for runtime in [Runtime::Node, Runtime::Tsx, Runtime::Bun] {
        LauncherPolicy::dev_server(runtime)
            .validate_args(&["src/server.ts"])
            .unwrap_or_else(|error| panic!("{runtime:?} must accept a script path: {error}"));
    }
}

/// Every runtime variant has a distinct name, so an error message cannot hide
/// which runtime was refused.
/// (Mọi variant có tên riêng để thông điệp lỗi không che runtime bị từ chối.)
#[test]
fn every_runtime_has_a_distinct_name() {
    let names = [
        Runtime::Bun.name(),
        Runtime::Deno.name(),
        Runtime::Node.name(),
        Runtime::Tsx.name(),
    ];
    let unique: std::collections::BTreeSet<&str> = names.iter().copied().collect();
    assert_eq!(
        unique.len(),
        names.len(),
        "duplicate runtime name: {names:?}"
    );
}

/// The Node flag profile is what protects `tsx` too — assert the policy the two
/// share is non-empty, so a future refactor cannot quietly empty it.
/// (Profile flag của Node cũng bảo vệ `tsx` — khẳng định nó không rỗng.)
#[test]
fn the_node_flag_profile_is_not_empty() {
    let error = LauncherPolicy::dev_server(Runtime::Node)
        .validate_args(&["-e", "x"])
        .expect_err("node -e must be refused");
    assert!(error.to_string().contains("node"), "{error}");
}
