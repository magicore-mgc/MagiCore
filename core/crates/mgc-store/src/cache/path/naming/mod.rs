//! Cache naming owner.
//! Owner naming cache.

mod naming;

pub(crate) use naming::{
    hash_version_identity, is_legacy_portable_name, is_legacy_portable_version,
};
