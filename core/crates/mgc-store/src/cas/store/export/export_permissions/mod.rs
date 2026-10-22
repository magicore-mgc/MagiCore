//! Export Permissions owner wiring.
//! Nối owner export permissions.

mod export_permissions;

pub(in crate::cas::store::export) use self::export_permissions::set_exported_permissions;

#[cfg(test)]
mod test;
