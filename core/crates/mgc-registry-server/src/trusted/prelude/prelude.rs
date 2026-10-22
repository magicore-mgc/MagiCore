//! Shared imports for trusted-publishing owners.
//! Import dùng chung giữa các owner trusted publishing.

pub(in crate::trusted::trusted) use crate::AppState;
pub(in crate::trusted::trusted) use anyhow::{Context, Result};
pub(in crate::trusted::trusted) use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Query, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post, put},
};
pub(in crate::trusted::trusted) use base64::Engine;
pub(in crate::trusted::trusted) use mgc_crypto::{Ed25519Signature, KeyPair};
pub(in crate::trusted::trusted) use mgc_oidc::OidcClaims;
pub(in crate::trusted::trusted) use serde::{Deserialize, Serialize};
pub(in crate::trusted::trusted) use sha2::Digest;
pub(in crate::trusted::trusted) use std::io::Write;
pub(in crate::trusted::trusted) use std::path::{Path, PathBuf};
