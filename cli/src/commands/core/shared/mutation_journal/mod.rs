//! Shared mutation journal facade.
//! Facade journal mutation dùng chung.

#[path = "mutation_journal.rs"]
mod implementation;

#[cfg(test)]
pub(crate) use implementation::manifest_canonical_digest;
pub(crate) use implementation::{
    MutationOperation, MutationSnapshot, atomic_write_file_checked,
    ensure_no_pending_remove_journal, finish_mutation_journal, journaled_step, record_post_image,
    refuse_project_link, rollback_mutation, stage_mutation_journal,
};
pub(crate) use implementation::{adopt_legacy_journal, recover_interrupted_remove};

#[cfg(test)]
#[path = "test_support/mod.rs"]
mod test_support;

#[cfg(test)]
pub(crate) use test_support::{copy_dir_recursive, craft_identity_journal, python_fixture};

#[cfg(all(test, feature = "lib"))]
pub(crate) use test_support::test_adapter_and_lock;
