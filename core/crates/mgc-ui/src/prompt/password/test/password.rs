//! Password prompt tests — kiểm thử prompt password.
//! This type contract does not claim a no-echo PTY test ran.
//! Type contract này không tuyên bố đã chạy PTY test no-echo.

use super::password::password;

#[test]
fn password_returns_an_anyhow_string_result() {
    let _password: fn(&str) -> anyhow::Result<String> = password;
}
