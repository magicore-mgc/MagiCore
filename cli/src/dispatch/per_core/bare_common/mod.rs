// Bare common adapter exports — expose the common-command mapper.
// Export của adapter bare common — cung cấp mapper lệnh dùng chung.
mod bare_common;

pub(in crate::dispatch) use bare_common::bare_common_to_common;
