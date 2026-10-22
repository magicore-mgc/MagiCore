//! Wire local HTTP tests for the advisory query owner.
//! Nối test HTTP cục bộ cho owner truy vấn advisory.

#![allow(clippy::unwrap_used)]

use super::*;
#[path = "query_maps_http_failure_to_scanner_failed.rs"]
mod query_maps_http_failure_to_scanner_failed;
#[path = "query_maps_malformed_json_to_hard_error.rs"]
mod query_maps_malformed_json_to_hard_error;
#[path = "query_parses_successful_response.rs"]
mod query_parses_successful_response;
mod support;
