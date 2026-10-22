//! Maven manifest owners / Owner manifest Maven.
mod parse_maven_manifest;
mod pom_dependency_is_mgc_managed;
mod prune_pom_dependencies;
mod upsert_pom_dependency;
mod write_pom_manifest;
pub(crate) use self::parse_maven_manifest::parse_maven_manifest;
pub(crate) use self::write_pom_manifest::write_pom_manifest;
