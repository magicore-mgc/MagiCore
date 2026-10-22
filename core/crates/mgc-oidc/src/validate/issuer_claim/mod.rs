//! Unverified issuer extraction owner — owner đọc issuer chưa xác minh.
//! Chỉ dùng claim để chọn endpoint đã allowlist; chữ ký JWT vẫn là căn cứ xác thực.

mod issuer_claim;
pub(crate) use issuer_claim::unverified_issuer;
