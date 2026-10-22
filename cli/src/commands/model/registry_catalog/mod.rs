// Wires the registry_catalog owner; behavior lives in registry_catalog.rs.
// Nối owner registry_catalog; xử lý nằm trong registry_catalog.rs.
mod error;
mod registry_catalog;
pub(super) use self::error::RegistryCatalogError;
pub(super) use self::registry_catalog::list;
