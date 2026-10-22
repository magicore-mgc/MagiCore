//! Container dispatch and default runtime pins.
//! Điều phối profile container và ghim runtime mặc định.

use anyhow::Result;
use std::path::Path;

use crate::scaffold::container::container_common::append_docker_readme;
use crate::scaffold::processor::Scaffolder;
use crate::wizard::engine::ScaffoldConfig;

use super::profiles::{
    write_ai_docker_assets, write_app_docker_assets, write_clo_docker_assets,
    write_lib_docker_assets, write_web_docker_assets, write_web_k8s_assets,
};

pub(super) const KUBERNETES_DNS_LABEL_MAX_BYTES: usize = 63;
pub(super) const KUBERNETES_RESOURCE_HASH_HEX_LENGTH: usize = 16;
pub(super) const DEFAULT_WEB_CONTAINER_PORT: u16 = 4315;
pub(super) const WEB_CONTAINER_UID: u32 = 65532;
pub(super) const WEB_CONTAINER_GID: u32 = 65532;
pub(super) const DEFAULT_CONTAINER_PROJECT_DIR: &str = "/app";
pub(super) const DEFAULT_CONTAINER_DATA_DIR: &str = "/mgc-data";
pub(super) const DEFAULT_CONTAINER_HOME_DIR: &str = "/tmp";
pub(super) const DEFAULT_CONTAINER_DATA_VOLUME: &str = "magicore-system-data";
pub(super) const DEFAULT_CONTAINER_ENTRYPOINT_FILE: &str = "mgc-container-entrypoint";
pub(super) const DEFAULT_CONTAINER_ENTRYPOINT_PATH: &str =
    "/usr/local/bin/mgc-container-entrypoint";
pub(super) const DEFAULT_MGC_IMAGE: &str =
    concat!("ghcr.io/mingd-153/magicore:", env!("CARGO_PKG_VERSION"));
pub(super) const DEFAULT_NODE_RUNTIME_IMAGE: &str = "node:26.12.0-trixie-slim";
pub(super) const DEFAULT_PYTHON_RUNTIME_IMAGE: &str = "python:3.15.0-slim-trixie";
pub(super) const DEFAULT_RUST_RUNTIME_IMAGE: &str = "rust:1.99.0-slim-trixie";
pub(super) const DEFAULT_GO_RUNTIME_IMAGE: &str = "golang:1.27.2-trixie";
pub(super) const DEFAULT_FLUTTER_VERSION: &str = "3.47.7";
pub(super) const DEFAULT_FLUTTER_ARCHIVE_SHA256: &str =
    "af7455f540df951184c7953fbe3cf884766e7541dacf8571f9f582db03dd6230";
pub(super) const DEFAULT_FLUTTER_RUNTIME_IMAGE: &str = "debian:trixie-slim";
pub(super) const DEFAULT_FLUTTER_RELEASE_BASE_URL: &str =
    "https://storage.googleapis.com/flutter_infra_release/releases/stable/linux";

pub(in crate::scaffold) fn generate_assets(target: &Path, config: &ScaffoldConfig) -> Result<()> {
    let docker_requested = config.features.iter().any(|feature| feature == "docker");
    let k8s_requested = config.features.iter().any(|feature| feature == "k8s");
    if !docker_requested && !k8s_requested {
        return Ok(());
    }
    if k8s_requested && config.core != "web" {
        return Err(crate::error::k8s_profile_unsupported(&config.core));
    }

    match config.core.as_str() {
        "web" => write_web_docker_assets(target, config)?,
        "ai" => write_ai_docker_assets(target, config)?,
        "clo" => write_clo_docker_assets(target, config)?,
        "lib" => write_lib_docker_assets(target, config)?,
        "app" => write_app_docker_assets(target, config)?,
        core => {
            return Err(super::error::container_profile_unsupported(
                core,
                &Scaffolder::framework(config),
                "no detected native project manifest",
            ));
        }
    }
    if docker_requested {
        append_docker_readme(target)?;
    }
    if k8s_requested {
        write_web_k8s_assets(target, config)?;
    }
    Ok(())
}
