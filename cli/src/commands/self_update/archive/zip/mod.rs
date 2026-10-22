//! Export safe ZIP binary extraction.
//! Công bố trích binary ZIP an toàn.

mod zip;
pub(in crate::commands::self_update) use zip::stage_from_zip;
