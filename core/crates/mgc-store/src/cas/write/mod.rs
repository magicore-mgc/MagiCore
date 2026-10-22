//! Shared CAS write facade for staging paths, file operations, and durability.
//! Facade ghi CAS dùng chung cho staging, thao tác file và độ bền dữ liệu.

mod error;
mod parent_directory_sync;
mod permissions;
mod temp_file_name;
mod unique_tmp_path;
mod write;

pub use self::parent_directory_sync::sync_parent_dir;
pub use self::temp_file_name::{TempFileName, TempPurpose};
pub use self::unique_tmp_path::unique_tmp_path;
pub use self::write::{
    STREAM_THRESHOLD, stream_write_verify_and_set_perms, write_all_verify_and_set_perms,
};
pub(in crate::cas) use self::write::{
    stream_write_verify_and_set_perms_mode, write_all_verify_and_set_perms_mode,
};
