//! Parse the existing truthy environment value set.
//! Parse tập giá trị truthy môi trường hiện hữu.

/// Read one environment variable using the guard's established truthy vocabulary.
/// Đọc một biến môi trường theo tập giá trị truthy đang dùng.
pub(in crate::supply_chain) fn env_truthy(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .is_some_and(|value| is_truthy_value(&value))
}

fn is_truthy_value(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
