//! Wires upgrades lockfile v2 records and provenance to v3.
//! Nối owner: Nâng record và provenance lockfile v2 lên v3.

mod v2_to_v3;
pub use self::v2_to_v3::migrate_v2_to_v3;
#[cfg(test)]
mod test;
