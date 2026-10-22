//! Wires finalizes and round-trip checks a lossless v4 document.
//! Nối owner: Hoàn thiện và kiểm tra round-trip tài liệu v4 không hao hụt.

mod finalize_migrated_v4;
pub use self::finalize_migrated_v4::finalize_migrated_v4;
#[cfg(test)]
mod test;
