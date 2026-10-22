//! Stages and atomically publishes a regular file opened without following its final link.
//! Ghi staging và publish atomic file thường được mở không follow link cuối.

mod error;
mod file_copy;
pub(crate) use error::FileCopyError;
pub(crate) use file_copy::copy_regular_file_no_follow;
