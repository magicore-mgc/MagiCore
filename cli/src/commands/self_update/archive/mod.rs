//! Declare archive-format and executable owners.
//! Khai báo owner định dạng archive và quyền thực thi.

mod archive;
mod executable;
mod tar_gz;
mod zip;
pub(in crate::commands) use archive::stage_binary;
pub(super) use executable::set_executable;
pub(super) use tar_gz::stage_from_tar_gz;
pub(super) use zip::stage_from_zip;
