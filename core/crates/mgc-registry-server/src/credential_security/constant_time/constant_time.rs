//! Constant-time equality for secret-derived digests.
//! So sánh hằng thời gian cho digest dẫn xuất từ secret.
//!
//! SEC-22/G2: comparing an admin token digest or a stored credential digest
//! with `==` short-circuits on the first differing byte, which leaks a byte
//! prefix oracle to a remote prober. This comparator always reads every byte
//! of the longer input, so its running time depends only on the input length.
//! (SEC-22/G2: so sánh digest admin token hoặc digest credential đã lưu bằng
//! `==` dừng ở byte lệch đầu tiên, làm lộ oracle tiền tố byte cho kẻ dò từ xa.
//! Bộ so sánh này luôn đọc mọi byte của input dài hơn, nên thời gian chạy chỉ
//! phụ thuộc độ dài.)
//!
//! This is what `subtle::ConstantTimeEq` does for the same reason; it is
//! re-implemented locally because `subtle` is only a transitive dependency and
//! this crate does not link it directly.
//! (Đây đúng là việc `subtle::ConstantTimeEq` làm; code viết lại cục bộ vì
//! `subtle` chỉ là dependency bắc cầu, crate này không link trực tiếp.)

use std::hint::black_box;

/// Compare two byte strings without data-dependent early exit.
/// So sánh hai chuỗi byte không dừng sớm phụ thuộc dữ liệu.
///
/// Length inequality is *not* a secret here (digests are fixed-width), so the
/// length check may exit early; the byte walk itself is unconditional.
/// (Chênh lệch độ dài không phải bí mật ở đây (digest có độ dài cố định), nên
/// kiểm tra độ dài được phép thoát sớm; vòng quét byte thì luôn chạy đủ.)
pub(crate) fn constant_time_eq(left: &str, right: &str) -> bool {
    let (left, right) = (left.as_bytes(), right.as_bytes());
    if left.len() != right.len() {
        return false;
    }
    let mut difference = 0u8;
    for index in 0..left.len() {
        difference |= left[index] ^ right[index];
    }
    // `black_box` stops the optimizer from turning the branchless OR loop into
    // an early-exit comparison.
    // (`black_box` ngăn optimizer biến vòng OR không nhánh thành so sánh thoát sớm.)
    black_box(difference) == 0
}

#[cfg(test)]
#[path = "test/constant_time.rs"]
mod tests;
