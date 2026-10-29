//! Project-manifest probe owner wiring — wiring owner probe manifest project.

#[path = "project_manifest.rs"]
mod implementation;

pub use implementation::declares_dotnet_project;
#[cfg(test)]
#[path = "test.rs"]
mod tests;
