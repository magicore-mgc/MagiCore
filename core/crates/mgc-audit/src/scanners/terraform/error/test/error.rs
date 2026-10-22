//! Test local Terraform parser error compatibility.
//! Kiểm tra tính tương thích lỗi parser Terraform cục bộ.
#![allow(clippy::unwrap_used)]

use super::TerraformLockError;

#[test]
fn malformed_provider_header_displays_the_legacy_message() {
    let error = TerraformLockError::MalformedProviderHeader {
        header: "provider \"broken".to_string(),
    };

    assert_eq!(
        error.to_string(),
        "malformed provider header in .terraform.lock.hcl: provider \"broken"
    );
}
