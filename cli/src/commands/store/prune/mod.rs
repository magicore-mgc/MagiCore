mod candidate_plan;
mod error;
mod prune;
mod root_safety;
pub(in crate::commands::store) use prune::prune_unreferenced;
pub(super) use prune::run_prune;
pub(in crate::commands::store) use root_safety::{
    existing_database_file, existing_store_cache_root, existing_vstore_root,
    validate_existing_lock_directory,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
