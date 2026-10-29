//! Dev runtime policy tests — test policy runtime dev.
//! The token detector decides which optimizer settings a dev script receives; a
//! runtime missing from the match silently gets none.
//! (Bộ detect token quyết định script dev nhận cấu hình optimizer nào; runtime
//! thiếu khỏi match sẽ không nhận gì.)

use super::{detect_runtime_from_tokens, validate_runtime_args};

#[test]
fn every_known_runtime_token_selects_a_runtime() {
    // Each token a `mgc.toml` dev script can plausibly start must resolve, so the
    // optimizer is loaded for it.
    // (Mọi token script dev có thể khởi chạy phải resolve để optimizer được nạp.)
    for token in [
        "bun",
        "deno",
        "node",
        "tsx",
        "npm",
        "pnpm",
        "yarn",
        "vite",
        "next",
        "webpack",
        "react-scripts",
    ] {
        let runtime = detect_runtime_from_tokens(&[token]);
        assert!(
            format!("{runtime:?}") != "Unknown",
            "'{token}' must select a runtime so its optimizer settings load"
        );
    }
}

#[test]
fn an_empty_token_list_is_unknown() {
    assert_eq!(
        format!("{:?}", detect_runtime_from_tokens(&[])),
        "Unknown",
        "an empty script has no runtime to detect"
    );
}

#[test]
fn an_unrelated_token_is_unknown_rather_than_guessed() {
    for token in ["python3", "cargo", "godot", "./run.sh"] {
        assert_eq!(
            format!("{:?}", detect_runtime_from_tokens(&[token])),
            "Unknown",
            "'{token}' is not a JavaScript runtime and must not be guessed as one"
        );
    }
}

#[test]
fn the_launcher_policy_rejects_inline_eval_on_every_js_runtime() {
    for runtime in ["node", "tsx", "bun"] {
        validate_runtime_args(runtime, &["-e", "code"])
            .expect_err(&format!("{runtime} -e must be refused"));
    }
}

#[test]
fn a_deno_permission_flag_is_refused() {
    validate_runtime_args("deno", &["--allow-run", "x"])
        .expect_err("deno --allow-run must be refused without an explicit grant");
}

#[test]
fn a_plain_script_argument_passes_for_every_runtime() {
    for runtime in ["node", "tsx", "bun", "deno"] {
        validate_runtime_args(runtime, &["src/dev.ts"])
            .unwrap_or_else(|error| panic!("{runtime} must accept a script path: {error}"));
    }
}
