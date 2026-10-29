//! `mgc run` flag-policy tests — test policy flag của `mgc run`.
//! `mgc run` starts a user-authored script, so it must apply the same flag policy
//! `mgc dev` uses. Before this, the identical `mgc.toml [scripts]` entry that
//! `mgc dev` refused executed arbitrary code through `mgc run`.
//! (`mgc run` khởi chạy script do người dùng viết nên phải dùng cùng policy flag
//! với `mgc dev`; trước đây cùng entry bị `mgc dev` từ chối lại chạy được qua
//! `mgc run`.)

use super::validate_runtime_flags;

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

/// Inline code execution is refused for every JavaScript runtime.
/// (Chạy mã inline bị từ chối cho mọi runtime JS.)
#[test]
fn inline_eval_is_refused_for_every_js_runtime() {
    for program in ["node", "tsx", "bun"] {
        for flag in ["-e", "--eval", "-p", "--print"] {
            assert!(
                validate_runtime_flags(program, &args(&[flag, "code"])).is_err(),
                "'{program} {flag}' must be refused by the run lane"
            );
        }
    }
}

/// Deno's permission flags stay refused.
/// (Flag permission của Deno vẫn bị từ chối.)
#[test]
fn deno_permission_flags_are_refused() {
    for flag in ["--allow-run", "--allow-net", "--allow-ffi"] {
        assert!(
            validate_runtime_flags("deno", &args(&[flag, "x"])).is_err(),
            "'deno {flag}' must be refused"
        );
    }
}

/// A script path is the supported way to run code, so it passes.
/// (Đường dẫn script là cách chạy code được hỗ trợ nên phải qua.)
#[test]
fn a_script_path_passes_for_every_runtime() {
    for program in ["node", "tsx", "bun", "deno"] {
        validate_runtime_flags(program, &args(&["scripts/build.ts"]))
            .unwrap_or_else(|error| panic!("'{program} <path>' must pass: {error}"));
    }
}

/// A non-JavaScript program is not this policy's business: `mgc-exec` owns its
/// allowlist and traversal rules.
/// (Chương trình không phải JS không thuộc policy này: `mgc-exec` sở hữu allowlist
/// và chống traversal của nó.)
#[test]
fn a_non_js_program_is_left_to_mgc_exec() {
    for program in ["cargo", "python3", "go", "dotnet", "php"] {
        validate_runtime_flags(program, &args(&["build"]))
            .unwrap_or_else(|error| panic!("'{program}' must not be policed here: {error}"));
    }
}

/// A flag is matched as a flag, not as a filename prefix.
///
/// Prefix matching refused a legitimate file named `-experimental.ts` (it starts
/// with `-e`) and accepted nothing new. The flag set decides danger; a filename
/// that merely starts with a flag's letters is not a flag.
/// (Prefix matching từ chối file tên `-experimental.ts` — chỉ vì bắt đầu bằng
/// `-e`. Tập flag quyết định nguy hiểm; tên file không phải flag.)
#[test]
fn a_filename_that_starts_with_a_flag_is_not_a_flag() {
    // `-experimental.ts` begins with `-e`, but Node treats a lone positional
    // starting with `-` as the script path only after `--`; without `--` it IS a
    // bundled short-option group, so the policy refuses it. The legitimate way to
    // run such a file is `-- -experimental.ts`, asserted below.
    // (`-experimental.ts` là cụm short-option khi không có `--`, nên policy từ
    // chối — cách đúng để chạy file đó là `--`.)
    assert!(
        validate_runtime_flags("node", &args(&["--", "-experimental.ts"])).is_ok(),
        "after `--` the token is positional and must pass"
    );
    assert!(
        validate_runtime_flags("node", &args(&["./src/-eval.ts"])).is_ok(),
        "a path whose first character is not a flag prefix passes"
    );
}

/// Module-loading and inline-value spellings are all refused.
/// (Mọi spelling nạp module và giá trị inline đều bị từ chối.)
#[test]
fn every_code_loading_spelling_is_refused() {
    for flag in [
        "--eval",
        "-e",
        "--print",
        "-p",
        "--import",
        "--require",
        "-r",
        "--loader",
        "--experimental-loader",
        "--permission",
    ] {
        assert!(
            validate_runtime_flags("node", &args(&[flag, "x"])).is_err(),
            "node {flag} must be refused"
        );
    }
}

/// The `--flag=value` form is refused too — the value is the dangerous part.
/// (Dạng `--flag=value` cũng bị từ chối vì giá trị là phần nguy hiểm.)
#[test]
fn the_equals_form_is_refused() {
    for arg in ["--eval=code", "--import=./x.mjs", "--require=evil"] {
        assert!(
            validate_runtime_flags("node", &args(&[arg])).is_err(),
            "'{arg}' must be refused"
        );
    }
}

/// Bundled short options are refused when the first letter is dangerous.
/// (Cụm short bị từ chối khi chữ đầu nguy hiểm.)
#[test]
fn bundled_short_options_are_refused() {
    assert!(validate_runtime_flags("node", &args(&["-ep", "1"])).is_err());
    assert!(validate_runtime_flags("node", &args(&["-pe", "1"])).is_err());
}
