// Wires the registry_client owner; behavior lives in registry_client.rs.
// Nối owner registry_client; xử lý nằm trong registry_client.rs.
mod registry_client;
pub(super) use self::registry_client::client;
