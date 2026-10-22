//! Prompt public API tests — kiểm thử API prompt công khai.
//! Function-item assignments pin each result type without simulating user input.
//! Gán function item giữ result type mà không giả lập input người dùng.

use super::{confirm, input, multi_select, password, select};

#[test]
fn public_prompt_functions_keep_their_result_contracts() {
    let _confirm: fn(&str) -> anyhow::Result<bool> = confirm;
    let _input: fn(&str) -> anyhow::Result<String> = input;
    let _password: fn(&str) -> anyhow::Result<String> = password;
    let _select: fn(&str, &[&str]) -> anyhow::Result<usize> = select;
    let _multi_select: fn(&str, &[&str]) -> anyhow::Result<Vec<usize>> = multi_select;
}
