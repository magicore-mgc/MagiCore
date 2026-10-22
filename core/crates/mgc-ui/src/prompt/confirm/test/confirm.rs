//! Confirm prompt tests — kiểm thử prompt xác nhận.
//! This compile-time contract does not claim terminal interaction coverage.
//! Contract compile-time này không tuyên bố đã kiểm tra tương tác terminal.

use super::confirm::confirm;

#[test]
fn confirm_returns_an_anyhow_boolean_result() {
    let _confirm: fn(&str) -> anyhow::Result<bool> = confirm;
}
