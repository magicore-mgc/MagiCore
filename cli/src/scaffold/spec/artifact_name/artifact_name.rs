//! Build a scaffold registry artifact name from its core and template name.
//! Tạo tên artifact registry scaffold từ core và tên template.

use super::super::CoreKind;

/// Return the existing `mgc-create-<core>-<name>` artifact naming convention.
/// Trả về quy ước tên artifact hiện tại `mgc-create-<core>-<name>`.
pub fn artifact_name(core: CoreKind, template_name: &str) -> String {
    format!("mgc-create-{}-{}", core.as_str(), template_name)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
