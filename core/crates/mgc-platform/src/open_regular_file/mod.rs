//! Secure final-component opening for regular files.
//! Mở an toàn file thường ở thành phần path cuối.

mod error;
mod open_regular_file;

pub use self::error::OpenRegularFileError;
pub use self::open_regular_file::open_regular_file_no_follow;
