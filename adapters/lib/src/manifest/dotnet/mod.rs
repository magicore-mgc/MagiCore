//! .NET manifest owners / Owner manifest .NET.
mod has_unmodeled_package_reference_semantics;
mod parse_csproj_manifest;
mod prune_csproj_references;
mod write_csproj_manifest;
pub(crate) use self::parse_csproj_manifest::parse_csproj_manifest;
pub(crate) use self::write_csproj_manifest::write_csproj_manifest;
