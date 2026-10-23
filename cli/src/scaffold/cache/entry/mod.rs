//! Cache entry owner.
//! Owner entry cache.

mod entry;

pub(crate) use entry::{layer_key_for, resolve_base, validate_existing_directory, validated_entry};
