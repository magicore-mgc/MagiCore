//! Native Web Docker and local Kubernetes development profile.
//! Profile Docker Web native và Kubernetes development.

use crate::scaffold::container::DEFAULT_CONTAINER_PROJECT_DIR;
use anyhow::Result;
use std::path::Path;

use crate::scaffold::container::container_common::{
    container_dockerignore, web_container_resource_name, write_container_entrypoint,
};
use crate::scaffold::container::{
    DEFAULT_CONTAINER_DATA_DIR, DEFAULT_CONTAINER_DATA_VOLUME, DEFAULT_CONTAINER_ENTRYPOINT_FILE,
    DEFAULT_CONTAINER_ENTRYPOINT_PATH, DEFAULT_CONTAINER_HOME_DIR, DEFAULT_GO_RUNTIME_IMAGE,
    DEFAULT_MGC_IMAGE, DEFAULT_NODE_RUNTIME_IMAGE, DEFAULT_PYTHON_RUNTIME_IMAGE,
    DEFAULT_RUST_RUNTIME_IMAGE, DEFAULT_WEB_CONTAINER_PORT, WEB_CONTAINER_GID, WEB_CONTAINER_UID,
};
use crate::scaffold::processor::Scaffolder;
use crate::wizard::engine::ScaffoldConfig;

pub(in crate::scaffold::container) fn write_web_docker_assets(
    target: &Path,
    config: &ScaffoldConfig,
) -> Result<()> {
    if matches!(config.sub_type.as_str(), "fullstack" | "monorepo") {
        return Err(super::error::web_container_runtime_split_unsupported());
    }
    let (runtime_image, runtime_env, is_node) = if target.join("package.json").is_file() {
        (DEFAULT_NODE_RUNTIME_IMAGE, "", true)
    } else if target.join("go.mod").is_file() {
        (
            DEFAULT_GO_RUNTIME_IMAGE,
            "ENV GOPROXY=off GOSUMDB=off GOTOOLCHAIN=local\n",
            false,
        )
    } else if target.join("Cargo.toml").is_file() {
        (
            DEFAULT_RUST_RUNTIME_IMAGE,
            "ENV CARGO_NET_OFFLINE=true\n",
            false,
        )
    } else if target.join("pyproject.toml").is_file() {
        (
            DEFAULT_PYTHON_RUNTIME_IMAGE,
            "ENV PYTHONDONTWRITEBYTECODE=1 PYTHONUNBUFFERED=1\n",
            false,
        )
    } else {
        return Err(super::error::container_profile_unsupported(
            "web",
            &Scaffolder::framework(config),
            "a native Node, Go, Rust, or PEP 621 Python dependency manifest",
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
         ENV MGC_DATA_DIR={DEFAULT_CONTAINER_DATA_DIR} HOME={DEFAULT_CONTAINER_HOME_DIR} SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt\n\
         {runtime_env}\
         COPY --chown={WEB_CONTAINER_UID}:{WEB_CONTAINER_GID} . .\n\
         RUN chmod 0555 {DEFAULT_CONTAINER_ENTRYPOINT_PATH}\n\
         RUN chown {WEB_CONTAINER_UID}:{WEB_CONTAINER_GID} {DEFAULT_CONTAINER_PROJECT_DIR} {DEFAULT_CONTAINER_DATA_DIR}\n\
         USER {WEB_CONTAINER_UID}:{WEB_CONTAINER_GID}\n\
         RUN mgc install-web --frozen\n\
         {node_test_step}\
         EXPOSE {DEFAULT_WEB_CONTAINER_PORT}\n\
         ENTRYPOINT [\"{DEFAULT_CONTAINER_ENTRYPOINT_PATH}\"]\n\
         CMD [\"dev\", \"--host\", \"0.0.0.0\", \"--port\", \"{DEFAULT_WEB_CONTAINER_PORT}\"]\n",
        node_test_step = if is_node { "RUN mgc test\n" } else { "" },
    );
    let image_name = format!("{}:local", web_container_resource_name(config));
    let compose = format!(
        "services:\n\
         \x20 app:\n\
         \x20\x20 build:\n\
         \x20\x20\x20 context: .\n\
         \x20\x20\x20 args:\n\
         \x20\x20\x20\x20 MGC_IMAGE: ${{MGC_IMAGE:-{mgc_image}}}\n\
         \x20\x20 image: ${{MGC_IMAGE_NAME:-{image_name}}}\n\
         \x20\x20 init: true\n\
         \x20\x20 ports:\n\
         \x20\x20\x20 - \"{DEFAULT_WEB_CONTAINER_PORT}:{DEFAULT_WEB_CONTAINER_PORT}\"\n\
         \x20\x20 volumes:\n\
         \x20\x20\x20 - mgc-data:{DEFAULT_CONTAINER_DATA_DIR}\n\
         \x20\x20 restart: unless-stopped\n\
         \n\
         volumes:\n\
         \x20 mgc-data:\n\
         \x20\x20 name: ${{MGC_DATA_VOLUME:-{DEFAULT_CONTAINER_DATA_VOLUME}}}\n"
    );
    // Use the shared list. This inline copy reproduced the first two lines of
    // `container_dockerignore()` by hand and drifted: it omitted `.dart_tool`,
    // `.flutter-plugins*`, `.gradle`, `Pods` and `build`, so a Web image built
    // from a polyglot repo sent stale `build/` output and Gradle caches into the
    // Docker context.
    // (Bản chép tay tái tạo hai dòng đầu của `container_dockerignore()` rồi lệch:
    // thiếu `.dart_tool`, `.flutter-plugins*`, `.gradle`, `Pods`, `build` — image
    // Web build từ repo đa ngôn ngữ đưa output `build/` cũ và cache Gradle vào
    // context Docker.)
    let dockerignore = container_dockerignore();

    Scaffolder::write_file(&target.join("Dockerfile"), &dockerfile)?;
    Scaffolder::write_file(&target.join(".dockerignore"), dockerignore)?;
    write_container_entrypoint(target, "mgc install-web")?;
    Scaffolder::write_file(&target.join("docker-compose.yml"), &compose)?;
    Ok(())
}

/// Emit a local-kind development Deployment using the image built by the
/// generated Dockerfile. This is explicitly a `mgc dev` lane, not a
/// production release manifest.
/// (Sinh Deployment cho local kind, chạy `mgc dev`; không gắn nhãn production.)
pub(in crate::scaffold::container) fn write_web_k8s_assets(
    target: &Path,
    config: &ScaffoldConfig,
) -> Result<()> {
    let name = web_container_resource_name(config);
    let image = format!("{name}:local");
    let probe_path = if target.join("package.json").is_file() {
        "/"
    } else {
        "/api/health"
    };
    let manifest = format!(
        r#"apiVersion: apps/v1
kind: Deployment
metadata:
  name: {name}
  labels:
    app.kubernetes.io/name: {name}
spec:
  replicas: 1
  selector:
    matchLabels:
      app.kubernetes.io/name: {name}
  template:
    metadata:
      labels:
        app.kubernetes.io/name: {name}
    spec:
      automountServiceAccountToken: false
      securityContext:
        runAsNonRoot: true
        runAsUser: {WEB_CONTAINER_UID}
        runAsGroup: {WEB_CONTAINER_GID}
        fsGroup: {WEB_CONTAINER_GID}
        seccompProfile:
          type: RuntimeDefault
      containers:
        - name: web
          image: {image}
          imagePullPolicy: IfNotPresent
          ports:
            - name: http
              containerPort: {DEFAULT_WEB_CONTAINER_PORT}
          env:
            - name: MGC_DATA_DIR
              value: {DEFAULT_CONTAINER_DATA_DIR}
          startupProbe:
            httpGet:
              path: {probe_path}
              port: http
            periodSeconds: 5
            failureThreshold: 36
          readinessProbe:
            httpGet:
              path: {probe_path}
              port: http
          livenessProbe:
            httpGet:
              path: {probe_path}
              port: http
          securityContext:
            allowPrivilegeEscalation: false
            capabilities:
              drop: [ALL]
---
apiVersion: v1
kind: Service
metadata:
  name: {name}
spec:
  selector:
    app.kubernetes.io/name: {name}
  ports:
    - name: http
      protocol: TCP
      port: {DEFAULT_WEB_CONTAINER_PORT}
      targetPort: http
  type: ClusterIP
"#
    );
    let readme = format!(
        "# Kubernetes development lane\n\nThis manifest runs `mgc dev` in a local cluster for development and E2E validation. It is not a production deployment profile.\n\nRun `mgc install` in the project first to create the lockfile consumed by the image. Build and load the image into kind, then apply the manifest:\n\n```sh\ndocker build -t {image} .\nkind load docker-image {image}\nkubectl apply -f k8s/deployment.yaml\n```\n\nThe generated image copies the project lock and runs `mgc install-web --frozen` before `mgc dev`. The pod runs as UID/GID 65532 with dropped Linux capabilities. Runtime MGC writes live in the pod's writable layer and are discarded when the pod is replaced.\n"
    );
    Scaffolder::write_file(&target.join("k8s").join("deployment.yaml"), &manifest)?;
    Scaffolder::write_file(&target.join("k8s").join("README.md"), &readme)?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
