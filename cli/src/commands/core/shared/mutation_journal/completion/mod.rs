//! Journal completion module wiring.
//! Wiring module hoàn tất journal.

#[path = "completion.rs"]
mod implementation;

pub(crate) use implementation::finish_mutation_journal;
pub(crate) use implementation::{clear_mutation_journal, mark_journal_completed};
