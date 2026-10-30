//! Project-name mapping tests — kiểm thử ánh xạ tên project.
//!
//! The mapping must be total and stable: every user-supplied name has to yield a
//! valid Python identifier and a PEP 503 distribution name.
//! Ánh xạ phải toàn phần và ổn định: mọi tên user nhập đều cho ra identifier
//! Python hợp lệ và tên distribution PEP 503.

use crate::scaffold::slug::{FALLBACK_MODULE, distribution_name, module_name};

#[test]
fn names_become_identifiers_and_distributions() {
    assert_eq!(module_name("my-agent"), "my_agent");
    assert_eq!(module_name("My Agent 2"), "my_agent_2");
    assert_eq!(distribution_name("My Agent 2"), "my-agent-2");
    assert_eq!(module_name("demo.app"), "demo_app");
    assert_eq!(distribution_name("demo.app"), "demo-app");
}

#[test]
fn digits_leading_names_are_prefixed_for_python() {
    assert_eq!(module_name("3d-agent"), "_3d_agent");
    assert_eq!(distribution_name("3d-agent"), "3d-agent");
}

#[test]
fn separator_only_names_fall_back() {
    assert_eq!(module_name("---"), FALLBACK_MODULE);
    assert_eq!(distribution_name("---"), "app");
    assert_eq!(module_name(""), FALLBACK_MODULE);
}
