//! Language projection wiring — nối projection language.
//! The leaf keeps only ecosystem results that differ from the base row.
//! File lá chỉ giữ kết quả ecosystem khác hàng cơ sở.

mod language_projection;

pub(super) use language_projection::language_projection;
