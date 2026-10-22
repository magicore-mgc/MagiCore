//! Deno lockfile owner — owner đọc lockfile Deno.
//! Owns strict JSON parsing of npm and JSR pin maps.
//! Sở hữu parse JSON nghiêm ngặt cho map pin npm và JSR.

mod deno_lock;
mod error;

pub use deno_lock::read_deno_lock;

#[cfg(test)]
#[path = "test/deno_lock.rs"]
mod tests;
