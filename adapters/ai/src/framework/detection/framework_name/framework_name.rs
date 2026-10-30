//! Canonical AI framework string parser.
//! Parser chuỗi framework AI chuẩn.

use super::super::AiFramework;

/// Parse one recognized framework name without normalizing unknown strings.
/// Parse tên framework được nhận diện, không chuẩn hóa chuỗi lạ.
pub(in crate::framework) fn framework_from_str(name: &str) -> Option<AiFramework> {
    // One parser owns the id table so the create lane and detection can never
    // disagree about which names are real.
    // (Một parser sở hữu bảng id để create lane và detection không bao giờ lệch
    // nhau về tên nào là thật.)
    AiFramework::parse(name)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
