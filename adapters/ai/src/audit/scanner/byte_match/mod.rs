//! Private byte-sequence matching owner for scanner signatures.
//! Owner nội bộ khớp chuỗi byte cho signature của scanner.

mod byte_match;

pub(super) use byte_match::contains_bytes;
