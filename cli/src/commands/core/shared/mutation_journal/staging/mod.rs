//! Journal staging module wiring.
//! Wiring module staging journal.

#[path = "staging.rs"]
mod implementation;

pub(crate) use implementation::{mutation_failpoint, record_post_image, stage_mutation_journal};
