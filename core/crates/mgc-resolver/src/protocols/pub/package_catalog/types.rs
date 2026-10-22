//! Typed pub.dev JSON response shapes.
//! Các type dữ liệu cho JSON response của pub.dev.

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub(in crate::protocols::pubdev) struct PubPackage {
    #[serde(default)]
    pub(super) versions: Vec<PubVersion>,
}

#[derive(Debug, Deserialize)]
pub(super) struct PubVersion {
    pub(super) version: String,
    #[serde(default)]
    pub(super) pubspec: Option<PubSpec>,
    #[serde(default)]
    pub(super) archive_url: String,
    #[serde(default)]
    pub(super) archive_sha256: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct PubSpec {
    #[serde(default)]
    pub(super) environment: Option<PubEnvironment>,
    #[serde(default)]
    pub(super) dependencies: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Default, Deserialize)]
pub(super) struct PubEnvironment {
    #[serde(default)]
    pub(super) sdk: String,
}
