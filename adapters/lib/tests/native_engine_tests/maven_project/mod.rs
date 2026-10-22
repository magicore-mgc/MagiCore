//! Maven project test owner facade.
//! Facade owner test maven project theo hành vi.

#[path = "maven_project.rs"]
mod implementation;

pub(crate) use implementation::lib_adapter_maven_pom_project_resolves_natively_via_env;
