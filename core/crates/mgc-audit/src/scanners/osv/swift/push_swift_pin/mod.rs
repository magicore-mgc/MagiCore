//! Build Swift OSV pin owner wiring.
//! File này chỉ nối module phân loại pin Swift.

mod push_swift_pin;
pub(in crate::scanners::osv) use push_swift_pin::push_swift_pin;
