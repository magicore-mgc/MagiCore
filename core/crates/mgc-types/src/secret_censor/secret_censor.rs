//! Apply the legacy literal replacement to a string.
//! Áp dụng phép thay thế literal tương thích lên chuỗi.

/// Replace four lowercase substrings without parsing keys or values.
/// Thay bốn chuỗi con chữ thường mà không parse khóa hay giá trị.
pub fn censor_secret(input: &str) -> String {
    input
        .replace("token", "****")
        .replace("password", "****")
        .replace("secret", "****")
        .replace("key", "****")
}
