//! Wires the private git_provenance owner modules.
//! Nối các module private thuộc owner git_provenance.

mod git_provenance;
pub(in crate::install) use git_provenance::git_commit_for_package;
