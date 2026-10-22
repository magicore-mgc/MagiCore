//! Next-steps tests — kiểm thử hướng dẫn bước tiếp.
//! The tested helper is consumed directly by print_next_steps.
//! Helper được kiểm tra là dữ liệu đầu vào trực tiếp của print_next_steps.

use super::next_steps::suggested_commands;

#[test]
fn suggestions_keep_project_path_and_install_development_commands() {
    let commands = suggested_commands("sample-app");
    assert_eq!(commands[0], "  cd sample-app");
    assert!(commands[1].contains("mgc install"));
    assert!(commands[2].contains("mgc dev"));
}
