#![cfg(test)]

//! Keep the hardware adapter marker constructible by its provider owners.
//! Giữ kiểu đánh dấu adapter hardware có thể được tạo bởi các owner provider.

use crate::adapter::adapter::HardwareAdapter;

#[test]
fn adapter_marker_remains_a_constructible_zero_sized_type() {
    let _adapter = HardwareAdapter;
    assert_eq!(std::mem::size_of::<HardwareAdapter>(), 0);
}
