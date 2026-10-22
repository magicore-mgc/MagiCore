// Wires the registry_pull owner; behavior lives in registry_pull.rs.
// Nối owner registry_pull; xử lý nằm trong registry_pull.rs.
mod registry_pull;
pub(super) use self::registry_pull::pull;
