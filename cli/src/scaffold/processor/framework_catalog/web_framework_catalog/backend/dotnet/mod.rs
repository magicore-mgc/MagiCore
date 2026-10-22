//! Backend dotnet framework owner wiring.
//! Wiring owner framework backend dotnet.

#[path = "dotnet.rs"]
mod implementation;

pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) use implementation::{
    DOTNET_MINIMAL, DOTNET_WEBAPI,
};
