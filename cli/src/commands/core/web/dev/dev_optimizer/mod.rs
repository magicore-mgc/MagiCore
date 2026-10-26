//! Dev optimizer profile owner.
//! Owner profile optimizer cho dev.

#[path = "dev_optimizer.rs"]
mod implementation;

#[cfg(test)]
pub(crate) use implementation::DevCompilerProfile;
pub(crate) use implementation::load;
