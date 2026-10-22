//! V4 reader wiring / Nối module reader v4.
mod v4_reader;
pub(crate) use v4_reader::{read_v4_document_if_present, verify_v4_document_integrity};
