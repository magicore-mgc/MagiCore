//! Cache command types owner wiring.
//! File này chỉ nối module types và export API nội bộ.

mod types;

pub(super) use types::{
    CacheAction, CacheEntry, CacheTarget, GenericPruneStats, WebProjectCacheStats,
    WebProjectPruneStats, WebSharedCacheStats,
};
