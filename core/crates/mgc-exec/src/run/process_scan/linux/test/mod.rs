//! Linux scan test facade.
//! Facade test scan Linux.
//!
//! `#[path]` keeps the test file a real module: `include!` pasted its `//!`
//! header after an item, which the newer compiler rejects as E0753.
//! (`#[path]` giữ file test là module thật: `include!` dán header `//!` sau một
//! item, và compiler mới từ chối bằng E0753.)
#[path = "linux.rs"]
mod linux;
