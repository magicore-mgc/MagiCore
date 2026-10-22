//! AI Python development container.
//! Container phát triển AI Python do MGC quản lý.

use crate::scaffold::container::DEFAULT_CONTAINER_PROJECT_DIR;
use anyhow::Result;
use std::path::Path;

use crate::scaffold::container::container_common::{
    container_compose, container_dockerignore, container_resource_name, write_container_entrypoint,
};
use crate::scaffold::container::{
    DEFAULT_CONTAINER_DATA_DIR, DEFAULT_CONTAINER_ENTRYPOINT_FILE,
    DEFAULT_CONTAINER_ENTRYPOINT_PATH, DEFAULT_CONTAINER_HOME_DIR, DEFAULT_MGC_IMAGE,
    DEFAULT_PYTHON_RUNTIME_IMAGE, WEB_CONTAINER_GID, WEB_CONTAINER_UID,
};
use crate::scaffold::processor::Scaffolder;
use crate::wizard::engine::ScaffoldConfig;

/// Generate the AI development container from the MGC scaffold itself.
/// Registry templates may add framework files, but a missing optional
/// template layer must never silently turn `--docker` into a no-op.
/// (Sinh container AI từ chính MGC scaffold. Thiếu template registry
/// tùy chọn không được khiến `--docker` âm thầm không làm gì.)
pub(in crate::scaffold::container) fn write_ai_docker_assets(
    target: &Path,
    config: &ScaffoldConfig,
) -> Result<()> {
    if !target.join("pyproject.toml").is_file() {
        return Err(super::error::container_manifest_required(
            "ai",
            &Scaffolder::framework(config),
            "pyproject.toml",
        ));
    }

    let mgc_image = DEFAULT_MGC_IMAGE;
    let dockerfile = format!(
        "ARG MGC_IMAGE={mgc_image}\n\
         ARG PYTHON_IMAGE={DEFAULT_PYTHON_RUNTIME_IMAGE}\n\
         FROM ${{MGC_IMAGE}} AS mgc-cli\n\
         FROM ${{PYTHON_IMAGE}}\n\
         WORKDIR {DEFAULT_CONTAINER_PROJECT_DIR}\n\
         COPY --from=mgc-cli /usr/local/bin/mgc /usr/local/bin/mgc\n\
         COPY --from=mgc-cli /usr/lib/libz.so.1 /usr/lib/libz.so.1\n\
         COPY --from=mgc-cli /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt\n\
         COPY --from=mgc-cli --chown={WEB_CONTAINER_UID}:{WEB_CONTAINER_GID} {DEFAULT_CONTAINER_DATA_DIR} {DEFAULT_CONTAINER_DATA_DIR}\n\
         COPY {DEFAULT_CONTAINER_ENTRYPOINT_FILE} {DEFAULT_CONTAINER_ENTRYPOINT_PATH}\n\
         RUN chmod 0555 {DEFAULT_CONTAINER_ENTRYPOINT_PATH}\n\
         COPY --chown={WEB_CONTAINER_UID}:{WEB_CONTAINER_GID} . .\n\
         ENV MGC_DATA_DIR={DEFAULT_CONTAINER_DATA_DIR} HOME={DEFAULT_CONTAINER_HOME_DIR} SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt PYTHONDONTWRITEBYTECODE=1\n\
         RUN chown {WEB_CONTAINER_UID}:{WEB_CONTAINER_GID} {DEFAULT_CONTAINER_PROJECT_DIR} {DEFAULT_CONTAINER_DATA_DIR}\n\
         USER {WEB_CONTAINER_UID}:{WEB_CONTAINER_GID}\n\
         RUN mgc install --frozen\n\
         RUN mgc test\n\
         RUN mgc build\n\
         ENTRYPOINT [\"{DEFAULT_CONTAINER_ENTRYPOINT_PATH}\"]\n\
         CMD [\"dev\"]\n"
    );
    let image_name = container_resource_name("ai", config);
    let compose = container_compose(&image_name, mgc_image, "dev");
    Scaffolder::write_file(&target.join("Dockerfile"), &dockerfile)?;
    Scaffolder::write_file(&target.join(".dockerignore"), container_dockerignore())?;
    write_container_entrypoint(target, "mgc install")?;
    Scaffolder::write_file(&target.join("docker-compose.yml"), &compose)?;
    Ok(())
}
