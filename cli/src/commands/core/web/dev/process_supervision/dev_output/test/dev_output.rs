//! Dev log line rendering tests — kiểm thử kết xuất dòng log dev.
//! Every line must name the role it belongs to, because both halves of a split
//! project print into the same terminal.
//! Mọi dòng phải nêu vai trò của nó, vì hai nửa của project tách rời cùng in vào
//! một terminal.

use super::super::{BACKEND_ROLE, backend_starting_line, readiness_line};
use crate::commands::core::dev_port::PORT_WEB_BE;

#[test]
fn readiness_line_names_role_and_endpoint() {
    assert_eq!(
        readiness_line(BACKEND_ROLE, "127.0.0.1", PORT_WEB_BE),
        format!("[backend] ready on http://127.0.0.1:{PORT_WEB_BE}")
    );
    assert_eq!(
        readiness_line("frontend", "localhost", 4315),
        "[frontend] ready on http://localhost:4315"
    );
}

#[test]
fn backend_starting_line_names_the_launcher() {
    let line = backend_starting_line(BACKEND_ROLE, "cargo");
    assert!(line.starts_with("[backend] "), "{line}");
    assert!(line.contains("cargo"), "{line}");
}
