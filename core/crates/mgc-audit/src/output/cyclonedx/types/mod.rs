//! Wire the public CycloneDX 1.5 serializable data model.
//! Nối data model CycloneDX 1.5 có thể serialize và được export public.

#[path = "types.rs"]
mod types;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub use self::types::*;
