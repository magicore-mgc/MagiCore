//! Cache staging and atomic publication owner.
//! Owner staging và publish nguyên tử cache.

mod entry;

pub(super) use entry::{
    TempPathGuard, create_unique_temp_file, publish_cache_entry, unique_temp_path,
};
