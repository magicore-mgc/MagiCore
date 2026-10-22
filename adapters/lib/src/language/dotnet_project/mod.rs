//! Root csproj discovery module wiring.
//! Wiring module tìm csproj ở root.

mod dotnet_project;

pub(crate) use dotnet_project::find_csproj;
