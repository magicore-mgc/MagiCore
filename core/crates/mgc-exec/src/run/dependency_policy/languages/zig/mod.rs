//! Zig direct-execution policy facade.
//! Facade policy chạy trực tiếp Zig.

#[path = "zig.rs"]
mod implementation;
pub(super) use implementation::denial_reason;
