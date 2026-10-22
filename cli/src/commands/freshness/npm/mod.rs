//! Npm owner exports — export owner npm.

mod npm;
pub use npm::{check_dev_npm_freshness_against_lock, declared_npm_dep_specs, declared_npm_deps};
