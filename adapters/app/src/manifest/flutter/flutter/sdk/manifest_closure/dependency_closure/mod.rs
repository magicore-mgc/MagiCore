//! SDK dependency-closure traversal owner wiring.
//! Wiring owner duyệt closure dependency SDK.

mod dependency_closure;
pub(in crate::manifest::flutter::flutter::sdk) use dependency_closure::read_flutter_sdk_dependency_closure;
