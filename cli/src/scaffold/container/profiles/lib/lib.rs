//! Multi-language library lifecycle container.
//! Container lifecycle thư viện đa ngôn ngữ.

use crate::scaffold::container::DEFAULT_CONTAINER_PROJECT_DIR;
use anyhow::Result;
use std::path::Path;

use crate::scaffold::container::container_common::{
    container_compose, container_dockerignore, container_resource_name, write_container_entrypoint,
};
use crate::scaffold::container::{
    DEFAULT_CONTAINER_DATA_DIR, DEFAULT_CONTAINER_ENTRYPOINT_FILE,
    DEFAULT_CONTAINER_ENTRYPOINT_PATH, DEFAULT_CONTAINER_HOME_DIR, DEFAULT_GO_RUNTIME_IMAGE,
    DEFAULT_MGC_IMAGE, DEFAULT_NODE_RUNTIME_IMAGE, DEFAULT_PYTHON_RUNTIME_IMAGE,
    DEFAULT_RUST_RUNTIME_IMAGE, WEB_CONTAINER_GID, WEB_CONTAINER_UID,
};
use crate::scaffold::processor::Scaffolder;
use crate::wizard::engine::ScaffoldConfig;

/// Generate a Rust library build/test image whose dependency lifecycle is
/// owned by MGC and whose compiler runs offline after `mgc install`.
/// (Sinh image test/build thư viện; MGC sở hữu dependency, toolchain chạy theo lock.)
pub(in crate::scaffold::container) fn write_lib_docker_assets(
    target: &Path,
    config: &ScaffoldConfig,
) -> Result<()> {
    let runtime_image = if target.join("Cargo.toml").is_file() {
        DEFAULT_RUST_RUNTIME_IMAGE
    } else if target.join("go.mod").is_file() {
        DEFAULT_GO_RUNTIME_IMAGE
    } else if target.join("pyproject.toml").is_file() {
        DEFAULT_PYTHON_RUNTIME_IMAGE
    } else if target.join("package.json").is_file() {
        DEFAULT_NODE_RUNTIME_IMAGE
    } else {
        return Err(super::error::container_profile_unsupported(
            "lib",
            &Scaffolder::framework(config),
            "no detected Rust, Go, Python, or Node manifest",
        ));
    };

    let mgc_image = DEFAULT_MGC_IMAGE;
    let dockerfile = format!(
        "ARG MGC_IMAGE={mgc_image}\n\
         ARG TOOLCHAIN_IMAGE={runtime_image}\n\
         FROM ${{MGC_IMAGE}} AS mgc-cli\n\
         FROM ${{TOOLCHAIN_IMAGE}}\n\
         WORKDIR {DEFAULT_CONTAINER_PROJECT_DIR}\n\
         COPY --from=mgc-cli /usr/local/bin/mgc /usr/local/bin/mgc\n\
         COPY --from=mgc-cli /usr/lib/libz.so.1 /usr/lib/libz.so.1\n\
         COPY --from=mgc-cli /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt\n\
         COPY --from=mgc-cli --chown={WEB_CONTAINER_UID}:{WEB_CONTAINER_GID} {DEFAULT_CONTAINER_DATA_DIR} {DEFAULT_CONTAINER_DATA_DIR}\n\
         COPY {DEFAULT_CONTAINER_ENTRYPOINT_FILE} {DEFAULT_CONTAINER_ENTRYPOINT_PATH}\n\
         RUN chmod 0555 {DEFAULT_CONTAINER_ENTRYPOINT_PATH}\n\
         COPY --chown={WEB_CONTAINER_UID}:{WEB_CONTAINER_GID} . .\n\
         ENV MGC_DATA_DIR={DEFAULT_CONTAINER_DATA_DIR} HOME={DEFAULT_CONTAINER_HOME_DIR} SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt\n\
         RUN chown {WEB_CONTAINER_UID}:{WEB_CONTAINER_GID} {DEFAULT_CONTAINER_PROJECT_DIR} {DEFAULT_CONTAINER_DATA_DIR}\n\
         USER {WEB_CONTAINER_UID}:{WEB_CONTAINER_GID}\n\
         RUN mgc install --frozen\n\
         RUN mgc test\n\
         RUN mgc build\n\
         ENTRYPOINT [\"{DEFAULT_CONTAINER_ENTRYPOINT_PATH}\"]\n\
         CMD [\"test\"]\n"
    );
    let image_name = container_resource_name("lib", config);
    let compose = container_compose(&image_name, mgc_image, "test");
    Scaffolder::write_file(&target.join("Dockerfile"), &dockerfile)?;
    Scaffolder::write_file(&target.join(".dockerignore"), container_dockerignore())?;
    write_container_entrypoint(target, "mgc install")?;
    Scaffolder::write_file(&target.join("docker-compose.yml"), &compose)?;
    Ok(())
}
