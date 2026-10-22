//! Dependency resolution engine — stable public facade for resolver owners.
//!
//! The provider contract, dependency-confusion audit, batch resolver, and PubGrub
//! engine live in named owner packages below this module. Their public paths are
//! re-exported here for compatibility.

mod dependency_confusion;
mod provider;
pub mod pubgrub;
mod resolver;

pub use dependency_confusion::{DepInfo, check_dependency_confusion};
pub use provider::{DependencyError, DependencyProvider, ResolvedDep};
pub use pubgrub::{Cause, Incompatibility, PubGrubSolver, SolveError as PubGrubSolveError, Term};
pub use resolver::{
    DedupePref, Resolution, Resolver, RootResolution, SolveError, SolveResult, SolveResultWithRoots,
};
