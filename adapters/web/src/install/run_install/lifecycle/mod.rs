//! Lifecycle policy and execution owner wiring.
//! Wiring owner policy và chạy lifecycle.

mod lifecycle;
pub(super) use lifecycle::run;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
