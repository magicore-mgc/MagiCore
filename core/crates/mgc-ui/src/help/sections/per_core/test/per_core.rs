//! Per Core help owner tests — test owner help per core.
//! Pin the nearest behavior contract without embedding test bodies in production files.
//! Giữ contract behavior gần owner, không đặt test body trong source production.

use super::append_per_core_section;

#[test]
fn per_core_section_retains_command_names_and_order() {
    let mut output = String::new();
    append_per_core_section(&mut output, &[("app", "App")]);

    let install = output.find("install-app").expect("install label exists");
    let add = output.find("add-app").expect("add label exists");
    let remove = output.find("remove-app").expect("remove label exists");
    let list = output.find("list-app").expect("list label exists");
    let update = output.find("update-app").expect("update label exists");
    assert!(install < add && add < remove && remove < list && list < update);
}
