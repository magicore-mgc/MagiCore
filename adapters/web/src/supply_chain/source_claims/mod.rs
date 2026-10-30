//! source_claims owner wiring / wiring owner source_claims.

#[path = "source_claims.rs"]
mod implementation;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(crate) use self::implementation::{
    RegistryOrigin, ensure_resolved_sources_satisfy_claims, project_lockfile_sources,
};
