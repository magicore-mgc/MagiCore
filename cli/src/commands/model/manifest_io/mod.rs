// Wires the manifest_io owner; behavior lives in manifest_io.rs.
// Nối owner manifest_io; xử lý nằm trong manifest_io.rs.
mod manifest_io;
#[cfg(test)]
pub(super) use self::manifest_io::save_manifest_in;
pub(super) use self::manifest_io::{now_iso, read_manifests_in, save_manifest};
