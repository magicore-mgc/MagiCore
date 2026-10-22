//! Tests for byte and path SHA-256 owners.
//! Test cho owner SHA-256 trên bytes và file path.

use super::*;

include!("sha256_bytes_matches_standard_digest/mod.rs");
include!("sha256_path_matches_bytes_digest/mod.rs");
include!("sha256_reader_matches_bytes_digest/mod.rs");
