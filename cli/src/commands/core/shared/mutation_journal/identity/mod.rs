//! Project and journal identity wiring.
//! Wiring identity project và journal.

#[path = "identity.rs"]
mod implementation;

pub(crate) use implementation::{
    adopt_legacy_journal, mutation_journal_dir, project_identity_id, read_project_identity_id,
};
