//! Cache root owner.
//! Owner root cache.

mod root;

pub(crate) use root::cache_root;
#[cfg(test)]
pub(crate) use root::cache_root_with_override;
