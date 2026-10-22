//! Registry server integration test owners grouped by protocol and behavior.

use super::*;

#[path = "access_policy.rs"]
mod access_policy;
#[path = "auth_service.rs"]
mod auth_service;
#[path = "model_storage.rs"]
mod model_storage;
#[path = "npm_routes.rs"]
mod npm_routes;
#[path = "oci_publish.rs"]
mod oci_publish;
#[path = "oci_routes.rs"]
mod oci_routes;
#[path = "pypi_upload.rs"]
mod pypi_upload;
#[path = "scope_glob.rs"]
mod scope_glob;
