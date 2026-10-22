//! Ecosystem detection exports — phát hiện core từ identity và manifest dự án.
//! Module này xuất API nhận diện core; không tự ghi cấu hình.
mod detect_ecosystem;
pub use detect_ecosystem::detect_ecosystem;
