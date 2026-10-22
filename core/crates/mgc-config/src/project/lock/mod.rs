//! Lockfile and writer-lock project settings.
//! Cấu hình lockfile và writer-lock của project.

#[path = "lock.rs"]
mod lock;

pub use lock::LockConfig;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
