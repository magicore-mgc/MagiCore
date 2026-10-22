//! Bounded per-package synchronization using fixed lock shards.
//! Đồng bộ package có giới hạn bằng các shard lock cố định.

use crate::package_cache::error::{CacheError, LockOperation};
use anyhow::Result;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Fixed lock count; collisions serialize unrelated packages but cannot grow memory.
/// Số lock cố định; collision chỉ tuần tự hóa package khác nhau, không làm tăng bộ nhớ.
const CACHE_LOCK_SHARDS: usize = 256;

pub(in crate::package_cache) struct PackageLocks {
    shards: Box<[RwLock<()>]>,
    hash_state: RandomState,
}

impl PackageLocks {
    pub(in crate::package_cache) fn new() -> Self {
        let shards = (0..CACHE_LOCK_SHARDS)
            .map(|_| RwLock::new(()))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self {
            shards,
            hash_state: RandomState::new(),
        }
    }

    pub(in crate::package_cache) fn read_for(
        &self,
        package_id: &str,
    ) -> Result<RwLockReadGuard<'_, ()>> {
        let index = self.shard_index(package_id);
        self.shards[index].read().map_err(|_| {
            CacheError::LockPoisoned {
                package_id: package_id.to_owned(),
                operation: LockOperation::Read,
            }
            .into()
        })
    }

    pub(in crate::package_cache) fn write_for(
        &self,
        package_id: &str,
    ) -> Result<RwLockWriteGuard<'_, ()>> {
        let index = self.shard_index(package_id);
        self.shards[index].write().map_err(|_| {
            CacheError::LockPoisoned {
                package_id: package_id.to_owned(),
                operation: LockOperation::Write,
            }
            .into()
        })
    }

    pub(in crate::package_cache) fn write_all(&self) -> Result<Vec<RwLockWriteGuard<'_, ()>>> {
        self.shards
            .iter()
            .enumerate()
            .map(|(index, shard)| {
                shard.write().map_err(|_| {
                    CacheError::LockPoisoned {
                        package_id: format!("shard-{index}"),
                        operation: LockOperation::Prune,
                    }
                    .into()
                })
            })
            .collect()
    }

    fn shard_index(&self, package_id: &str) -> usize {
        let mut hasher = self.hash_state.build_hasher();
        for byte in package_id.bytes() {
            let normalized = match byte {
                b'/' | b'\\' | b':' => b'_',
                _ => byte.to_ascii_lowercase(),
            };
            normalized.hash(&mut hasher);
        }
        (hasher.finish() as usize) % self.shards.len()
    }
}
