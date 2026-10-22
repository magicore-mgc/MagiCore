//! Wiring for the .NET + Rust audit plan.
//! Wiring cho plan audit .NET + Rust.

mod dotnet_rust;
pub(in crate::audit) use dotnet_rust::aggregate_dotnet_rust;
