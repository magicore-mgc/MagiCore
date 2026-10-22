//! Shared install timing diagnostics.
//! Diagnostic thời gian install dùng chung.

#[path = "install_profile.rs"]
mod implementation;

pub(crate) use implementation::profile_install_mark;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
