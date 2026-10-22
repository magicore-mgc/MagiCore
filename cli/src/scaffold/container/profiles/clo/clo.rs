//! Cloud CDK validation container.
//! Container kiểm tra Cloud CDK do MGC quản lý.

use crate::scaffold::container::DEFAULT_CONTAINER_PROJECT_DIR;
use anyhow::Result;
use std::path::Path;

use crate::scaffold::container::container_common::{
    container_compose, container_dockerignore, container_resource_name, write_container_entrypoint,
};
use crate::scaffold::container::{
    DEFAULT_CONTAINER_DATA_DIR, DEFAULT_CONTAINER_ENTRYPOINT_FILE,
    DEFAULT_CONTAINER_ENTRYPOINT_PATH, DEFAULT_CONTAINER_HOME_DIR, DEFAULT_MGC_IMAGE,
    DEFAULT_NODE_RUNTIME_IMAGE, WEB_CONTAINER_GID, WEB_CONTAINER_UID,
};
use crate::scaffold::processor::Scaffolder;
use crate::wizard::engine::ScaffoldConfig;

/// Generate a CDK validation image whose dependencies and synth lifecycle
/// remain owned by MagiCore; it does not deploy to a cloud account.
/// (Sinh image kiểm thử CDK; dependency/build qua MGC và không deploy lên cloud.)
pub(in crate::scaffold::container) fn write_clo_docker_assets(
    target: &Path,
    config: &ScaffoldConfig,
) -> Result<()> {
    let framework = Scaffolder::framework(config);
    if !matches!(framework.as_str(), "cdk" | "cdk-typescript")
        || !target.join("package.json").is_file()
        || !target.join("cdk.json").is_file()
    {
        return Err(super::error::container_profile_unsupported(
            "clo",
            &framework,
            "verified CDK TypeScript manifests (package.json and cdk.json)",
        ));
    }

    let mgc_image = DEFAULT_MGC_IMAGE;
    let dockerfile = format!(
        "ARG MGC_IMAGE={mgc_image}\n\
         ARG NODE_IMAGE={DEFAULT_NODE_RUNTIME_IMAGE}\n\
         FROM ${{MGC_IMAGE}} AS mgc-cli\n\
         FROM ${{NODE_IMAGE}}\n\
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
         CMD [\"build\"]\n"
    );
    let image_name = container_resource_name("clo", config);
    let compose = container_compose(&image_name, mgc_image, "build");
    Scaffolder::write_file(&target.join("Dockerfile"), &dockerfile)?;
    Scaffolder::write_file(&target.join(".dockerignore"), container_dockerignore())?;
    write_container_entrypoint(target, "mgc install")?;
    Scaffolder::write_file(&target.join("docker-compose.yml"), &compose)?;
    Ok(())
}
