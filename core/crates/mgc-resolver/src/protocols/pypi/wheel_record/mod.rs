#[path = "archive_index/mod.rs"]
pub(super) mod archive_index;
#[path = "wheel_record.rs"]
mod implementation;
#[path = "record_csv/mod.rs"]
pub(super) mod record_csv;
#[path = "record_paths/mod.rs"]
pub(super) mod record_paths;
#[path = "record_verification/mod.rs"]
pub(super) mod record_verification;
#[path = "runtime_verification/mod.rs"]
pub(super) mod runtime_verification;

#[cfg(test)]
#[path = "test_support/mod.rs"]
pub(in crate::protocols::pypi) mod test_support;
