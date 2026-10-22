//! Install Options owner contract test.
//! Kiểm tra hợp đồng tùy chọn cài đặt.

use crate::adapter::InstallOptions;

#[test]
fn install_defaults_do_not_authorize_lifecycle_scripts() {
    let options = InstallOptions::default();
    assert!(!options.allow_scripts);
    assert!(options.force_install.is_empty());
    assert!(!options.replay_from_lock);
}
