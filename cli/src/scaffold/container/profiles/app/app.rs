//! Flutter application build and test container.
//! Container build và test ứng dụng Flutter.

use crate::scaffold::container::DEFAULT_CONTAINER_PROJECT_DIR;
use anyhow::Result;
use std::path::Path;

use crate::scaffold::container::container_common::{
    container_compose, container_dockerignore, container_resource_name, write_container_entrypoint,
};
use crate::scaffold::container::{
    DEFAULT_CONTAINER_DATA_DIR, DEFAULT_CONTAINER_ENTRYPOINT_FILE,
    DEFAULT_CONTAINER_ENTRYPOINT_PATH, DEFAULT_CONTAINER_HOME_DIR, DEFAULT_FLUTTER_ARCHIVE_SHA256,
    DEFAULT_FLUTTER_RELEASE_BASE_URL, DEFAULT_FLUTTER_RUNTIME_IMAGE, DEFAULT_FLUTTER_VERSION,
    DEFAULT_MGC_IMAGE, WEB_CONTAINER_GID, WEB_CONTAINER_UID,
};
use crate::scaffold::processor::Scaffolder;
use crate::wizard::engine::ScaffoldConfig;

/// Generate the verified Flutter web-test container with MGC owning Pub dependencies.
/// Sinh container Flutter web-test đã xác minh; MGC sở hữu dependency Pub.
pub(in crate::scaffold::container) fn write_app_docker_assets(
    target: &Path,
    config: &ScaffoldConfig,
) -> Result<()> {
    let framework = Scaffolder::framework(config);
    if framework != "flutter" || !target.join("pubspec.yaml").is_file() {
        return Err(super::error::container_manifest_required(
            "app",
            &framework,
            "pubspec.yaml (Flutter profile)",
        ));
    }

    let mgc_image = DEFAULT_MGC_IMAGE;
    let dockerfile = format!(
        "ARG MGC_IMAGE={mgc_image}\n\
         ARG FLUTTER_IMAGE={DEFAULT_FLUTTER_RUNTIME_IMAGE}\n\
         ARG FLUTTER_VERSION={DEFAULT_FLUTTER_VERSION}\n\
         ARG FLUTTER_ARCHIVE_SHA256={DEFAULT_FLUTTER_ARCHIVE_SHA256}\n\
         ARG FLUTTER_RELEASE_BASE_URL={DEFAULT_FLUTTER_RELEASE_BASE_URL}\n\
         FROM ${{MGC_IMAGE}} AS mgc-cli\n\
         FROM ${{FLUTTER_IMAGE}}\n\
         ARG FLUTTER_VERSION\n\
         ARG FLUTTER_ARCHIVE_SHA256\n\
         ARG FLUTTER_RELEASE_BASE_URL\n\
         RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl git unzip xz-utils zip libglu1-mesa && rm -rf /var/lib/apt/lists/*\n\
         RUN curl --fail --location --silent --show-error \"${{FLUTTER_RELEASE_BASE_URL}}/flutter_linux_${{FLUTTER_VERSION}}-stable.tar.xz\" --output /tmp/flutter.tar.xz && echo \"${{FLUTTER_ARCHIVE_SHA256}}  /tmp/flutter.tar.xz\" | sha256sum --check --strict - && tar -xJf /tmp/flutter.tar.xz -C /opt && rm /tmp/flutter.tar.xz\n\
         RUN chown -R {WEB_CONTAINER_UID}:{WEB_CONTAINER_GID} /opt/flutter\n\
         ENV PATH=/opt/flutter/bin:$PATH HOME={DEFAULT_CONTAINER_HOME_DIR}\n\
         USER {WEB_CONTAINER_UID}:{WEB_CONTAINER_GID}\n\
         RUN /opt/flutter/bin/flutter config --no-analytics && /opt/flutter/bin/flutter precache --web\n\
         USER 0:0\n\
         WORKDIR {DEFAULT_CONTAINER_PROJECT_DIR}\n\
         COPY --from=mgc-cli /usr/local/bin/mgc /usr/local/bin/mgc\n\
         COPY --from=mgc-cli /usr/lib/libz.so.1 /usr/lib/libz.so.1\n\
         COPY --from=mgc-cli /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt\n\
         COPY --from=mgc-cli --chown={WEB_CONTAINER_UID}:{WEB_CONTAINER_GID} {DEFAULT_CONTAINER_DATA_DIR} {DEFAULT_CONTAINER_DATA_DIR}\n\
         COPY {DEFAULT_CONTAINER_ENTRYPOINT_FILE} {DEFAULT_CONTAINER_ENTRYPOINT_PATH}\n\
         COPY --chown={WEB_CONTAINER_UID}:{WEB_CONTAINER_GID} . .\n\
         ENV PATH=/opt/flutter/bin:$PATH MGC_DATA_DIR={DEFAULT_CONTAINER_DATA_DIR} HOME={DEFAULT_CONTAINER_HOME_DIR} SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt\n\
         RUN chmod 0555 {DEFAULT_CONTAINER_ENTRYPOINT_PATH}\n\
         RUN chown {WEB_CONTAINER_UID}:{WEB_CONTAINER_GID} {DEFAULT_CONTAINER_PROJECT_DIR} {DEFAULT_CONTAINER_DATA_DIR}\n\
         USER {WEB_CONTAINER_UID}:{WEB_CONTAINER_GID}\n\
         RUN mgc install --frozen && mgc test && mgc build\n\
         ENTRYPOINT [\"{DEFAULT_CONTAINER_ENTRYPOINT_PATH}\"]\n\
         CMD [\"test\"]\n"
    );
    let image_name = container_resource_name("app", config);
    let compose = container_compose(&image_name, mgc_image, "test");
    Scaffolder::write_file(&target.join("Dockerfile"), &dockerfile)?;
    Scaffolder::write_file(&target.join(".dockerignore"), container_dockerignore())?;
    write_container_entrypoint(target, "mgc install")?;
    Scaffolder::write_file(&target.join("docker-compose.yml"), &compose)?;
    Ok(())
}
