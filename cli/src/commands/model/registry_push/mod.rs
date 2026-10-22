// Wires the registry_push owner; behavior lives in registry_push.rs.
// Nối owner registry_push; xử lý nằm trong registry_push.rs.
mod error;
mod registry_push;
pub(super) use self::error::RegistryPushError;
pub(super) use self::registry_push::push;
