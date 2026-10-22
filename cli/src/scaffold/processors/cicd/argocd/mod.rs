//! Argo CD output owner wiring.
//! Wiring owner output Argo CD.

#[path = "argocd.rs"]
mod implementation;
pub(super) use implementation::write;
