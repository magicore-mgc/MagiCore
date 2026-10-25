//! Host-tool runner owner.
//! Owner chạy chương trình host.

#[path = "host_tool.rs"]
mod implementation;

pub(crate) use implementation::run;
