//! CocoaPods dependency-edge owner wiring.
//! Nối owner cạnh dependency CocoaPods.

#[path = "dependencies.rs"]
mod implementation;

pub(super) use implementation::spec_dependencies;
