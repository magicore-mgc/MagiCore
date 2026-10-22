//! Text-input prompt tests — kiểm thử prompt nhập văn bản.
//! This type contract does not simulate terminal input.
//! Type contract này không giả lập input terminal.

use super::input::input;

#[test]
fn input_returns_an_anyhow_string_result() {
    let _input: fn(&str) -> anyhow::Result<String> = input;
}
