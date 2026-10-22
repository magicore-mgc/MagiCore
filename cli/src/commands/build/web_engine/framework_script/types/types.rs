//! Shared typed output for validated framework build launches.
//! Kiểu output dùng chung cho launch framework đã được xác thực.

use std::ffi::OsString;
use std::path::PathBuf;

/// A concrete executable, argument vector and extra environment for one launch.
/// Executable, vector argument và environment bổ sung của một launch.
pub(crate) type BuildLaunch = (PathBuf, Vec<OsString>, Vec<(OsString, OsString)>);
