//! Library project manifest compatibility and language owners.
//! Compatibility surface stays stable while parser/writer behavior lives by ecosystem.
mod cargo;
mod dotnet;
mod go;
mod maven;
mod python;
pub(crate) use cargo::{parse_cargo_manifest, write_cargo_manifest};
pub(crate) use dotnet::{parse_csproj_manifest, write_csproj_manifest};
pub(crate) use go::{parse_go_mod_manifest, write_go_mod_manifest};
pub(crate) use maven::{parse_maven_manifest, write_pom_manifest};
pub use python::supports_native_python_project;
pub(crate) use python::{parse_pyproject_manifest, write_pyproject_manifest};
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
