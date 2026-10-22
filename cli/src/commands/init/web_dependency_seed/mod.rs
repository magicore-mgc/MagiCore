//! Web dependency-seed owner exports.
//! Export owner bổ sung manifest Web.

mod web_dependency_seed;

pub(in crate::commands::init) use web_dependency_seed::seed_web_deps;
