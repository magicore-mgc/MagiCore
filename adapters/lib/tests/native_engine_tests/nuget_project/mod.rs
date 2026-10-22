//! Nuget project test owner facade.
//! Facade owner test nuget project theo hành vi.

#[path = "nuget_project.rs"]
mod implementation;

pub(crate) use implementation::lib_adapter_csproj_project_resolves_natively_via_env;
