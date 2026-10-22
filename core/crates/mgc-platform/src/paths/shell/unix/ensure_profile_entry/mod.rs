//! Unix shell profile update owner.
//! Owner cập nhật profile shell Unix.

mod ensure_profile_entry;

pub(super) use ensure_profile_entry::ensure_profile_entry;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
