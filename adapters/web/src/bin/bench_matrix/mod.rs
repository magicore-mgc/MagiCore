// Web benchmark module wiring — exports nội bộ cho binary benchmark Web.
// Keep behavior in named owners and expose only the binary entrypoint — chỉ nối module.
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use flate2::Compression;
use flate2::write::GzEncoder;
use mgc_store::{Layout, PackageCache};
use mgc_types::capabilities::{ContentStoreProvider, DependencyResolver};
use mgc_types::{PackageAdapter, PackageId, PackageName, ResolvedGraph, ResolvedPackage, Version};
use mgc_web_adapter::WebAdapter;
use sha2::{Digest, Sha512};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use tar::{Builder, Header};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use walkdir::WalkDir;

pub(super) mod baseline;
pub(super) mod cli;
pub(super) mod fixture;
pub(super) mod matrix;
pub(super) mod metrics;
pub(super) mod report;
pub(super) mod scenario;
pub(super) mod storage;
pub(super) mod types;
use baseline::*;
pub(super) use cli::run;
use fixture::*;
use matrix::*;
use metrics::*;
use report::*;
use scenario::*;
use storage::*;
use types::*;
