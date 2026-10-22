#[path = "error.rs"]
mod error;
use error::LibBuildError;

#[path = "lib.rs"]
mod implementation;
#[path = "manifest/mod.rs"]
mod manifest;

pub(super) use implementation::build_lib;
