//! Request timeout and response-size defaults — timeout và giới hạn body mặc định.
//!
/// Shared request limits — giới hạn chung cho request định danh.
pub const REQUEST_TIMEOUT_SECS: u64 = 15;
pub const MAX_RESPONSE_BYTES: usize = 1024 * 1024;
