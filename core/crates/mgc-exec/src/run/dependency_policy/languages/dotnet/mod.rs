//! .NET package and restore-target policy facade.
//! Facade policy package và restore target .NET.

#[path = "dotnet.rs"]
mod implementation;
pub(super) use implementation::denial_reason;
