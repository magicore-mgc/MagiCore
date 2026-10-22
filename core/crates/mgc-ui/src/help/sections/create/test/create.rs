//! Create help owner tests — test owner help create.
//! Pin the nearest behavior contract without embedding test bodies in production files.
//! Giữ contract behavior gần owner, không đặt test body trong source production.

use super::append_create_section;

#[test]
fn create_section_uses_web_shortcut_or_core_names() {
    let mut web = String::new();
    append_create_section(&mut web, &[("web", "Web")], true);
    assert!(web.contains("Scaffold a new web project"));
    assert!(!web.contains("create-web"));

    let mut scoped = String::new();
    append_create_section(&mut scoped, &[("app", "App")], false);
    assert!(scoped.contains("create-app"));
    assert!(scoped.contains("Scaffold App"));
}
