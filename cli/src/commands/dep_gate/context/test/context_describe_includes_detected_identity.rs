//! Context identity formatting — định dạng identity context.
//! Keeps audit descriptions precise when optional evidence exists.
//! Giữ mô tả audit chính xác khi có bằng chứng tùy chọn.

use crate::commands::dep_gate::{DepContext, DepOp};

#[test]
fn describe_includes_ecosystem_framework_target_and_manifest() {
    let context = DepContext::new(
        "iot",
        Some("esp32-rust"),
        Some("esp32"),
        Some("esp32dev"),
        DepOp::Install,
    )
    .with_manifest_format(Some("cargo-toml"));

    assert_eq!(
        context.describe(),
        "iot[esp32-rust|esp32|target:esp32dev|manifest:cargo-toml]"
    );
    assert_eq!(
        DepContext::new("web", None, None, None, DepOp::Install).describe(),
        "web"
    );
}
