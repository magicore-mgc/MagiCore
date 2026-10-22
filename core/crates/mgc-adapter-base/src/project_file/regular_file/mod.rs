//! Wiring for checked regular-file reads.
//! Nối các thao tác đọc file thường có kiểm tra.

mod regular_file;
pub(super) use self::regular_file::{read_regular_text, reject_non_regular};
