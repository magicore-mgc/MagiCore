//! Metadata-store test and shared fixture wiring.
//! Wiring test metadata store và fixture dùng chung.

#![allow(clippy::unwrap_used, clippy::await_holding_lock)]

#[path = "metadata_fallback.rs"]
mod metadata_fallback;
#[path = "metadata_schema.rs"]
mod metadata_schema;
#[path = "../../test/support.rs"]
#[allow(clippy::duplicate_mod)]
// Shared fixture source is compiled into each cache owner test module.
mod support;
