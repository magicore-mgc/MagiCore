#![allow(clippy::unwrap_used)]

use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

#[path = "common/mod.rs"]
mod common;

struct RegistryFixture {
    _server: mockito::ServerGuard,
    _mocks: Vec<mockito::Mock>,
    url: String,
}

impl RegistryFixture {
    fn new(latest: &str, versions: &[&str]) -> Self {
        let mut server = mockito::Server::new();
        let url = server.url();
        let versions_json = versions
            .iter()
            .map(|version| ((*version).to_string(), package_metadata(&url, version)))
            .collect::<serde_json::Map<_, _>>();
        let metadata = serde_json::json!({
            "name": "mgc-fixture",
            "dist-tags": { "latest": latest },
            "versions": versions_json
        });
        let metadata_mock = server
            .mock("GET", "/mgc-fixture")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(metadata.to_string())
            .expect_at_least(1)
            .create();
        let mut mocks = vec![metadata_mock];
        for version in versions {
            mocks.push(
                server
                    .mock(
                        "GET",
                        format!("/mgc-fixture/-/mgc-fixture-{version}.tgz").as_str(),
                    )
                    .with_status(200)
                    .with_header("content-type", "application/octet-stream")
                    .with_body(package_tarball(version))
                    .expect_at_least(1)
                    .create(),
            );
        }
        Self {
            _server: server,
            _mocks: mocks,
            url,
        }
    }
}

fn package_metadata(registry_url: &str, version: &str) -> serde_json::Value {
    serde_json::json!({
        "name": "mgc-fixture",
        "version": version,
        "dependencies": {},
        "dist": {
            "tarball": format!("{registry_url}/mgc-fixture/-/mgc-fixture-{version}.tgz")
        }
    })
}

fn package_tarball(version: &str) -> Vec<u8> {
    let package_json = serde_json::json!({
        "name": "mgc-fixture",
        "version": version,
        "main": "index.js"
    })
    .to_string();
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut archive = tar::Builder::new(encoder);
    append_tar_file(
        &mut archive,
        "package/package.json",
        package_json.as_bytes(),
    );
    append_tar_file(
        &mut archive,
        "package/index.js",
        b"module.exports = 'fixture';\n",
    );
    archive.finish().unwrap();
    archive.into_inner().unwrap().finish().unwrap()
}

fn append_tar_file(
    archive: &mut tar::Builder<flate2::write::GzEncoder<Vec<u8>>>,
    path: &str,
    content: &[u8],
) {
    let mut header = tar::Header::new_gnu();
    header.set_mode(0o644);
    header.set_size(content.len() as u64);
    header.set_cksum();
    archive.append_data(&mut header, path, content).unwrap();
}

fn run_mgc(
    binary: &str,
    args: &[&str],
    cwd: &Path,
    home: &Path,
    data: &Path,
    registry: Option<&str>,
) -> String {
    let mut command = Command::new(binary);
    command
        .args(args)
        .current_dir(cwd)
        .env("HOME", home)
        .env("USERPROFILE", home)
        .env("MGC_DATA_DIR", data)
        .env("MGC_CACHE_DIR", cwd.join(".magicore"))
        .env("MGC_TEMPLATES_DIR", data.join("templates"))
        .env("MGC_SCAFFOLDS_DIR", data.join("scaffolds"))
        .env("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST", "1")
        .env("CARGO_NET_OFFLINE", "true");
    if let Some(registry) = registry {
        command
            .env("MAGICORE_WEB_REGISTRY_URL", registry)
            .env("MAGICORE_WEB_ALLOWED_REGISTRIES", registry);
    }
    let output = command.output().expect("spawn mgc");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.status.success(), "mgc {args:?} failed:\n{text}");
    text
}

#[test]
fn every_claimed_web_js_framework_runs_add_update_remove_and_install() {
    let binary = env!("CARGO_BIN_EXE_mgc");
    let project_root = TempDir::new().unwrap();
    let home = TempDir::new().unwrap();
    let data = home.path().join("data");
    let add_registry = RegistryFixture::new("1.0.0", &["1.0.0"]);
    let update_registry = RegistryFixture::new("1.1.0", &["1.0.0", "1.1.0"]);

    for framework in [
        "vanilla",
        "ts",
        "node",
        "angular",
        "astro",
        "express",
        "nestjs",
        "nextjs",
        "nuxt",
        "qwik",
        "trpc",
        "solidjs",
        "sveltekit",
        "remix",
        "react-vite",
        "vue-vite",
    ] {
        let name = format!("package-ops-{framework}");
        // Supply framework source from an existing project of the same framework.
        // Cung cấp source framework từ project có sẵn cùng framework.
        let source = common::local_source(project_root.path(), "web", framework);
        run_mgc(
            binary,
            &["create-web", framework, &name, "--ts", &source.flag()],
            project_root.path(),
            home.path(),
            &data,
            None,
        );
        let project = project_root.path().join(&name);
        // The scaffold must be a real project from the framework's own
        // initializer, not a MagiCore-authored starter: the manifest and at
        // least one source file have to exist before package operations run.
        // Scaffold phải là project thật từ initializer của chính framework,
        // không phải starter do MagiCore viết: manifest và ít nhất một file
        // source phải tồn tại trước khi chạy thao tác package.
        let manifest_path = project.join("package.json");
        assert!(
            manifest_path.is_file(),
            "{framework}: scaffold produced no package.json"
        );
        let manifest: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&manifest_path).unwrap())
                .expect("scaffold package.json must be valid JSON");
        assert!(
            manifest["name"].is_string() || manifest["version"].is_string(),
            "{framework}: scaffold package.json is not a real project manifest: {manifest}"
        );
        let has_source = walkdir_source_exists(&project);
        assert!(
            has_source,
            "{framework}: scaffold produced no source files besides metadata"
        );
        // Keep the generated framework identity while isolating package operations
        // from unrelated template dependencies.
        std::fs::write(
            project.join("package.json"),
            format!("{{\"name\":\"{name}\",\"version\":\"1.0.0\",\"dependencies\":{{}}}}\n"),
        )
        .unwrap();

        run_mgc(
            binary,
            &["add", "mgc-fixture@^1.0.0"],
            &project,
            home.path(),
            &data,
            Some(&add_registry.url),
        );
        let manifest: serde_json::Value =
            serde_json::from_slice(&std::fs::read(project.join("package.json")).unwrap()).unwrap();
        assert_eq!(
            manifest["dependencies"]["mgc-fixture"], "^1.0.0",
            "{framework}: add must update the project manifest"
        );
        let lock = std::fs::read_to_string(project.join("mgc.lock")).unwrap();
        assert!(
            lock.contains("1.0.0"),
            "{framework}: add must lock the resolved version"
        );
        assert!(
            project
                .join("node_modules/mgc-fixture/package.json")
                .is_file(),
            "{framework}: add must install the resolved package"
        );

        run_mgc(
            binary,
            &["update", "mgc-fixture"],
            &project,
            home.path(),
            &data,
            Some(&update_registry.url),
        );
        let lock = std::fs::read_to_string(project.join("mgc.lock")).unwrap();
        assert!(
            lock.contains("1.1.0"),
            "{framework}: update must select latest in range"
        );
        assert!(
            project
                .join("node_modules/mgc-fixture/package.json")
                .is_file()
        );

        run_mgc(
            binary,
            &["remove", "mgc-fixture"],
            &project,
            home.path(),
            &data,
            Some(&update_registry.url),
        );
        let manifest: serde_json::Value =
            serde_json::from_slice(&std::fs::read(project.join("package.json")).unwrap()).unwrap();
        assert!(manifest["dependencies"].get("mgc-fixture").is_none());
        assert!(!project.join("node_modules/mgc-fixture").exists());

        run_mgc(
            binary,
            &["install", "--frozen"],
            &project,
            home.path(),
            &data,
            Some(&update_registry.url),
        );
    }
}

/// Whether the project contains at least one non-metadata source file.
/// Project có ít nhất một file source không phải metadata không.
fn walkdir_source_exists(project: &std::path::Path) -> bool {
    let metadata = ["package.json", "mgc.toml", "mgc.lock", ".mgc.core"];
    fn visit(dir: &std::path::Path, metadata: &[&str]) -> bool {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return false;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|n| n == "node_modules") {
                    continue;
                }
                if visit(&path, metadata) {
                    return true;
                }
            } else if path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|name| !metadata.contains(&name))
            {
                return true;
            }
        }
        false
    }
    visit(project, &metadata)
}
