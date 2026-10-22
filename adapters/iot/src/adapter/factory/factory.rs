//! Constructs the adapter from the existing framework detector and preserves `None` for a non-IoT project. — Tạo adapter từ framework detector hiện tại và giữ `None` cho project không phải IoT.

use std::path::Path;

use super::super::IotAdapter;
use crate::framework::detect_framework;
pub fn adapter_for(root: &Path) -> Option<IotAdapter> {
    let framework = detect_framework(root)?;
    Some(IotAdapter { framework })
}
