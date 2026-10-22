//! Normalize template names for existing artifact lookup behavior.
//! Chuẩn hóa tên template theo hành vi tra artifact hiện có.

/// Lowercase, replace underscores with hyphens, and trim surrounding whitespace.
/// Viết thường, thay dấu gạch dưới bằng gạch ngang và trim khoảng trắng hai đầu.
pub(in crate::scaffold::spec) fn normalize_template_name(name: &str) -> String {
    name.to_lowercase().replace('_', "-").trim().to_string()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
