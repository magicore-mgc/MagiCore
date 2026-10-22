//! npm-compatible private registry API owners.
//! Các owner API registry riêng tương thích npm.

mod batch_publish;
mod dist_tags;
mod npm;
mod package;
mod routes;
mod scope;
mod search;
mod tarball;
mod user;

pub use npm::routes;

#[cfg(test)]
use crate::model::Package;
#[cfg(test)]
use package::rewrite_tarball_host;
#[cfg(test)]
#[path = "../test/npm.rs"]
mod tarball_scheme_tests;
