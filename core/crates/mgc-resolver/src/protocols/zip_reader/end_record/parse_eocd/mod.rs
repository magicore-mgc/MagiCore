//! EOCD field decoder facade — wiring cho bộ giải mã trường EOCD.

mod parse_eocd;

pub(super) use parse_eocd::parse_eocd;
