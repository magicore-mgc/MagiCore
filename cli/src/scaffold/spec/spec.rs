//! Parsed scaffold request shared by all supported cores.
//! Request scaffold đã parse, dùng chung cho các core.

use super::{CoreKind, ScaffoldRef};

/// Parsed scaffold request.
/// Yêu cầu scaffold đã được phân tích.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldSpec {
    /// Core kind (web, ai, app, lib).
    /// Loại core (web, ai, app, lib).
    pub core: CoreKind,
    /// Original template name from user input.
    /// Tên template gốc từ đầu vào người dùng.
    pub name: String,
    /// Normalized name used for artifact lookup.
    /// Tên đã chuẩn hóa để tra artifact.
    pub normalized_name: String,
    /// Requested version, tag, or range.
    /// Version, tag hoặc range được yêu cầu.
    pub requested_ref: ScaffoldRef,
}

impl ScaffoldSpec {
    /// Preserve an explicit registry ref while using the normalized template name.
    /// Giữ reference registry tường minh khi dùng tên template đã chuẩn hóa.
    pub(crate) fn template_request(&self) -> String {
        match &self.requested_ref {
            ScaffoldRef::Default => self.normalized_name.clone(),
            ScaffoldRef::Version(reference)
            | ScaffoldRef::DistTag(reference)
            | ScaffoldRef::Range(reference) => {
                format!("{}@{reference}", self.normalized_name)
            }
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
