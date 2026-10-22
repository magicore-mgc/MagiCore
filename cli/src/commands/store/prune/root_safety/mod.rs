//! Validates project Web store and virtual-store roots.
//! Xác thực root Web store và virtual store theo project.

mod error;
mod root_safety;
pub(in crate::commands::store) use error::RootSafetyError;
pub(in crate::commands::store) use root_safety::{
    existing_database_file, existing_store_cache_root, existing_vstore_root,
    validate_existing_lock_directory,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
