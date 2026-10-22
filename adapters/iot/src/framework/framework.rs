//! Canonicalize a supported IoT framework alias.
//! Chuẩn hóa alias framework IoT được hỗ trợ.

use crate::framework::IotFramework;

pub(in crate::framework) fn canonical_framework(framework: &str) -> Option<&'static str> {
    IotFramework::from_str(framework).map(|parsed| parsed.as_str())
}

#[cfg(test)]
#[path = "framework/test/mod.rs"]
mod tests;
