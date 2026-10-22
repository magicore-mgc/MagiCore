#![allow(clippy::unwrap_used)]

use mgc_store::PackageCache;
use mgc_types::PackageId;
use std::sync::{Arc, Barrier};

include!("package_cache_concurrency/test/mod.rs");
