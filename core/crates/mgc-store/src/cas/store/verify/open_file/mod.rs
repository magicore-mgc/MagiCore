//! Maps the shared platform opener into the CAS verification error contract.
//! Ánh xạ helper mở file của platform vào hợp đồng lỗi xác minh CAS.

mod error;
mod open_file;
pub(in crate::cas::store) use self::open_file::open_regular_file;
