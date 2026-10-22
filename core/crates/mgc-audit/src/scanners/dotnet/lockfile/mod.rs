//! Wire NuGet lock parsing and its nearest-owner tests.
//! Nối parse lock NuGet và test gần owner.

mod lockfile;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub use self::lockfile::read_packages_lock;
