//! Full v4 document IO owner wiring.
//! File này chỉ export API đọc/ghi document v4.

mod document_io;

pub use document_io::{parse_v4_document, write_v4_document};
