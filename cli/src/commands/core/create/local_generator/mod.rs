//! Offline local scaffold generators wired from the per-core adapters.
//! Local generator scaffold offline nối từ adapter theo core.
//!
//! The adapter crates ship real, tested file generators (IoT boards, Bevy, the
//! Cloud providers, and the AI Python skeletons). They write files only — they
//! never spawn an external toolchain — so they are the offline path when neither
//! the framework's own initializer nor a registry layer is available.
//! Các crate adapter có generator file thật, đã test (board IoT, Bevy, provider
//! Cloud, skeleton Python AI). Chúng chỉ ghi file — không spawn toolchain ngoài
//! — nên đây là đường offline khi không có initializer của framework lẫn
//! registry layer.

mod local_generator;

pub(crate) use local_generator::{LocalScaffoldRequest, try_local_scaffold};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
