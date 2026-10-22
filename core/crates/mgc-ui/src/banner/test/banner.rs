//! Banner behavior tests — kiểm thử hành vi banner.
//! Tests stay beside the owner and are not embedded in production source.
//! Test nằm cạnh owner, không đặt body trong source production.

use super::banner::banner_display;

#[test]
fn banner_contains_product_and_package_manager_label() {
    let rendered = banner_display().to_string();
    assert!(rendered.contains("MagiCore"));
    assert!(rendered.contains("Universal Package Manager"));
}
