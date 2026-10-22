//! Wires detects the lockfile schema version from toml.
//! Nối owner: Nhận diện phiên bản schema lockfile từ TOML.

mod version_detection;
pub use self::version_detection::detect_lockfile_version;
#[cfg(test)]
mod test;
