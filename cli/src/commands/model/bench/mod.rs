//! ai bench owner wiring.
//! Wiring owner ai bench.

#[path = "bench.rs"]
mod implementation;

pub(in crate::commands::model) use implementation::run;
#[cfg(test)]
pub(in crate::commands::model) use implementation::{
    CANDIDATE_RUNTIMES, RuntimeLatency, measure_runtime,
};
