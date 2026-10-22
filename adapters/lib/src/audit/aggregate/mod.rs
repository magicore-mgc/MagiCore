//! Wiring for language-specific Lib + Rust audit plans.
//! Wiring cho plan audit Lib + Rust theo ngôn ngữ.

mod dotnet_rust;
mod go_rust;
mod java_rust;
mod python_rust;

pub(super) use dotnet_rust::aggregate_dotnet_rust;
pub(super) use go_rust::aggregate_go_rust;
pub(super) use java_rust::aggregate_java_rust;
pub(super) use python_rust::aggregate_python_rust;
