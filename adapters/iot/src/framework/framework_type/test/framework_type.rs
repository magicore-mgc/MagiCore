#![cfg(test)]

//! Verify framework aliases and their canonical string labels.
//! Xác minh alias framework và nhãn string canonical.

use crate::framework::IotFramework;

#[test]
fn framework_strings_keep_existing_aliases() {
    assert_eq!(
        IotFramework::from_str("esp32-rust"),
        Some(IotFramework::Esp32Rust)
    );
    assert_eq!(
        IotFramework::from_str("platformio"),
        Some(IotFramework::Platformio)
    );
    assert_eq!(
        IotFramework::from_str("zephyr-arm"),
        Some(IotFramework::Zephyr)
    );
    assert_eq!(IotFramework::Zephyr.as_str(), "zephyr");
    assert_eq!(IotFramework::from_str("other"), None);
}
