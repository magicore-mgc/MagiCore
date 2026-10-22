// Per-core common adapter exports — expose the common CLI mapper.
// Export của adapter common theo core — cung cấp mapper CLI common.
mod common;

pub(in crate::dispatch) use common::common_to_dispatch;
