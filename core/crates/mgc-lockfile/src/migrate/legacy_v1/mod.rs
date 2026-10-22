//! Wires owns legacy-v1 lockfile records and parsing.
//! Nối owner: Sở hữu record và parser lockfile v1 cũ.

mod legacy_v1;
pub use self::legacy_v1::{LockfileV1, PackageV1, parse_lockfile_v1};
#[cfg(test)]
mod test;
