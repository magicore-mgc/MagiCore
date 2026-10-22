//! Options help owner tests — test owner help options.
//! Pin the nearest behavior contract without embedding test bodies in production files.
//! Giữ contract behavior gần owner, không đặt test body trong source production.

use super::append_options_section;

#[test]
fn options_section_renders_core_help_and_version_flags() {
    let mut output = String::new();
    append_options_section(&mut output, &[("web", "Web"), ("app", "App")]);

    assert!(output.contains("Target core (web, app)"));
    assert!(output.contains("-h, --help"));
    assert!(output.contains("-V, --version"));
}
