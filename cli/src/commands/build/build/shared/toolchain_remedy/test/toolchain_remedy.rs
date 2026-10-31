//! Toolchain remedy classification tests — kiểm phân loại khắc phục toolchain.
//!
//! Each case pins the exact remedy sentence a user sees. A missing Rust target
//! or a missing Python build backend must name the command that fixes it, and a
//! plain compile error must stay untouched: mislabelling a code error as a setup
//! problem sends the user to reinstall a toolchain that was never broken.
//! Mỗi ca ghim đúng câu khắc phục mà user thấy. Thiếu Rust target hay Python
//! build backend phải nêu đúng lệnh sửa, còn lỗi biên dịch thường phải giữ
//! nguyên: gắn nhãn sai lỗi code thành lỗi cài đặt sẽ khiến user cài lại
//! toolchain không hề hỏng.

use super::super::{ToolchainContext, toolchain_remedy, with_remedy};

/// Real Cargo output for a target whose std is not installed.
/// Output Cargo thật khi std của target chưa được cài.
const CARGO_TARGET_MISSING: &str = "error[E0463]: can't find crate for `core`\n  |\n  = note: the `aarch64-unknown-linux-gnu` target may not be installed\n  = help: consider downloading the target with `rustup target add aarch64-unknown-linux-gnu`";

/// Real Python output when `build` is absent from the interpreter.
/// Output Python thật khi interpreter thiếu `build`.
const PYTHON_BACKEND_MISSING: &str = "/usr/bin/python3: No module named build";

const PLAIN_COMPILE_ERROR: &str = "error[E0425]: cannot find value `total` in this scope";

#[test]
fn missing_rust_target_names_the_exact_rustup_command() {
    let remedy = toolchain_remedy(
        CARGO_TARGET_MISSING,
        ToolchainContext {
            rust_target: Some("aarch64-unknown-linux-gnu"),
            python: None,
        },
    )
    .expect("missing target is classified");
    assert!(
        remedy.contains("rustup target add aarch64-unknown-linux-gnu"),
        "remedy must name the exact triple: {remedy}"
    );

    // Without a known triple the remedy still names the command shape, never a
    // guessed triple that could add the wrong std.
    // (Không biết triple, câu khắc phục vẫn nêu dạng lệnh, không đoán triple
    // có thể cài sai std.)
    let generic = toolchain_remedy(CARGO_TARGET_MISSING, ToolchainContext::default())
        .expect("missing target is classified even without a triple");
    assert!(generic.contains("rustup target add <target-triple>"));
}

#[test]
fn missing_python_build_backend_names_the_install_command() {
    let remedy = toolchain_remedy(
        PYTHON_BACKEND_MISSING,
        ToolchainContext {
            rust_target: None,
            python: Some("python3"),
        },
    )
    .expect("missing build backend is classified");
    assert!(
        remedy.contains("python3 -m pip install build"),
        "remedy must name the interpreter that ran: {remedy}"
    );

    let quoted = toolchain_remedy(
        "Traceback (most recent call last):\nModuleNotFoundError: No module named 'build'",
        ToolchainContext::default(),
    )
    .expect("quoted module name is classified");
    assert!(quoted.contains("python -m pip install build"));
}

#[test]
fn compile_errors_are_not_dressed_up_as_setup_problems() {
    assert_eq!(
        toolchain_remedy(
            PLAIN_COMPILE_ERROR,
            ToolchainContext {
                rust_target: Some("x86_64-unknown-linux-gnu"),
                python: Some("python3"),
            }
        ),
        None,
        "an ordinary compile error must reach the user unchanged"
    );
}

#[test]
fn with_remedy_keeps_the_original_message_first() {
    let message = "'cargo' exited with code 101 (812 ms)".to_string();
    let rendered = with_remedy(
        message.clone(),
        CARGO_TARGET_MISSING,
        ToolchainContext {
            rust_target: Some("aarch64-unknown-linux-gnu"),
            python: None,
        },
    );
    assert!(rendered.starts_with(&message), "detail comes first");
    assert!(rendered.contains("rustup target add aarch64-unknown-linux-gnu"));

    assert_eq!(
        with_remedy(
            message.clone(),
            PLAIN_COMPILE_ERROR,
            ToolchainContext::default()
        ),
        message,
        "no remedy means the caller's message is unchanged"
    );
}
