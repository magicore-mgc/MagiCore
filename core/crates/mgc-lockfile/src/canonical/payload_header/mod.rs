//! Writes version, generator and source tables before every other canonical section.
//! File này chỉ nối owner payload_header.

mod payload_header;

pub(crate) use payload_header::append_payload_header;
