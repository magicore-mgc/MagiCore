#![allow(clippy::unwrap_used)]
//! File-backed package-cache contracts. / Hợp đồng đọc package cache dạng file.

use mgc_store::PackageCache;
use mgc_types::PackageId;

include!("package_cache_streaming/test/mod.rs");
