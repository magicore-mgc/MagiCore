//! Publish test owner wiring and shared context.
//! Đăng ký test publish và khai báo ngữ cảnh import dùng chung.

use super::{
    GitReadQuery, OCI_UPLOAD_CHUNK_BYTES, TrustedPublishSession, docker_push_timeout,
    effective_trusted_oci_repository, git_exec_options, oci_request_timeout,
    parse_git_behind_count, publish_lifecycle_decision, publish_trusted_oci_artifacts,
    safe_oci_upload_location, trusted_npm_request_timeout, trusted_oci_artifact_manifest,
    trusted_token_deadline, trusted_token_expires_within,
};
use axum::Router;
use axum::body::Bytes;
use axum::extract::{Path as AxumPath, Query, State};
use axum::http::{HeaderMap, Method, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};

mod git_checks;
mod lifecycle_policy;
mod registry_policy;
mod trusted_oci_upload;
