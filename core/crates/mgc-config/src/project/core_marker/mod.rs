//! Core-marker owner module wiring.
//! Wiring các module sở hữu vòng đời core marker.
mod core_marker;
mod error;
mod identity;
mod reader;
mod writer;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
