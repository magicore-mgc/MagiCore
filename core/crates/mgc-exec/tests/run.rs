#![allow(clippy::unwrap_used)]
//! Run tests — dry-run không spawn, audit log ghi args đã REDACTED, exit ≠ 0 bail
//! (00-index §5.5 dry-run, §5.4 audit, §5.8 fail → bail)

#[path = "run/mod.rs"]
mod run;
