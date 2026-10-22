//! End-of-central-directory parsing facade — wiring cho EOCD.

mod end_record;
mod find_eocd;
mod parse_eocd;

pub(super) use end_record::read_end_record;
