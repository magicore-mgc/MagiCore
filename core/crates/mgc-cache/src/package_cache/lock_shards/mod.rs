//! Fixed-size package lock-shard exports.
//! Export các shard lock package có số lượng cố định.

mod lock_shards;

pub(super) use lock_shards::PackageLocks;
