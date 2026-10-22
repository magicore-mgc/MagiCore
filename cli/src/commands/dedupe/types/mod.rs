//! Dedupe request/result type wiring — preserves stable public type paths.
//! Wiring kiểu request/result dedupe — giữ đường dẫn type public ổn định.
mod types;
pub use types::DedupeEntry;
pub use types::{DedupeArgs, DedupeReport};
