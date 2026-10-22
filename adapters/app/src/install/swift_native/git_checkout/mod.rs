//! Wires the private git_checkout owner modules.
//! Nối các module private thuộc owner git_checkout.

mod git_checkout;
pub(in crate::install) use git_checkout::materialize_git_checkout;
