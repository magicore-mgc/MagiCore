//! OWASP advisory text truncation owner wiring.
//! File này chỉ nối module truncate_utf8.

mod truncate_utf8;

pub(super) use truncate_utf8::truncate_utf8;
