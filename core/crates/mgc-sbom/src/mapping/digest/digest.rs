//! Hex digest shape validation — Xác thực hình dạng digest hex

pub(crate) fn is_hex_digest(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
