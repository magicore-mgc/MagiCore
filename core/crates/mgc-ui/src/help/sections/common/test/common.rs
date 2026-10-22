//! Common help owner tests — test owner help common.
//! Pin the nearest behavior contract without embedding test bodies in production files.
//! Giữ contract behavior gần owner, không đặt test body trong source production.

use super::append_common_section;

#[test]
fn common_section_selects_generic_or_scoped_install_label() {
    let mut generic = String::new();
    append_common_section(&mut generic, &[("web", "Web")], true);
    assert!(generic.contains("Install all dependencies"));

    let mut scoped = String::new();
    append_common_section(&mut scoped, &[("web", "Web")], false);
    assert!(scoped.contains("install-web"));
    assert!(scoped.contains("Install dependencies for Web"));
}
