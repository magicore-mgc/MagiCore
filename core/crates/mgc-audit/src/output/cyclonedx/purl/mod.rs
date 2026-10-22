//! Wire package URL mapping and purl reference helpers.
//! Nối mapping package URL và helper dựng purl reference.

#[path = "purl.rs"]
mod purl;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(super) use self::purl::{component_bom_ref, purl_name, purl_type_for};
