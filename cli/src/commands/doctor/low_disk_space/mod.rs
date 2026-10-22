//! Low-disk threshold and truthful remediation advice.
//! Ngưỡng disk thấp và hướng xử lý trung thực.

mod low_disk_space;
#[cfg(test)]
pub(super) use low_disk_space::LOW_DISK_WARNING_THRESHOLD_BYTES;
pub(super) use low_disk_space::advice;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
