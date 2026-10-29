//! Real CLI installs retain npmrc credential scopes across V3/V4 and Web/Lib routes.
//! Khóa lỗi rò token bằng CLI thật, fixture cục bộ và cache riêng cho từng ca.
#![cfg(all(feature = "web", feature = "lib"))]
#![allow(clippy::unwrap_used)]
use base64::Engine;
use sha2::Digest;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;
const PORTS: &[u16] = &[
    4315, 4351, 4135, 4153, 4513, 4531, 3415, 3451, 3145, 3154, 3541, 3514, 1345, 1354, 1435, 1453,
    1534, 1543, 5134, 5143, 5314, 5341, 5413, 5431,
];
const MAX_HEADERS: usize = 16 * 1024;
const IO_TIMEOUT: Duration = Duration::from_secs(5);
const POLL_INTERVAL: Duration = Duration::from_millis(5);
const NAME: &str = "scope-fixture";
fn archive() -> Vec<u8> {
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut tar = tar::Builder::new(encoder);
    let manifest = serde_json::json!({"name":NAME,"version":"1.0.0","main":"index.js"}).to_string();
    for (name, bytes) in [
        ("package/package.json", manifest.as_bytes()),
        ("package/index.js", b"module.exports = 42;\n".as_slice()),
    ] {
        let mut h = tar::Header::new_gnu();
        h.set_size(bytes.len() as u64);
        h.set_mode(0o644);
        h.set_cksum();
        tar.append_data(&mut h, name, bytes).unwrap();
    }
    tar.into_inner().unwrap().finish().unwrap()
}
struct Fixture {
    url: String,
    seen: Arc<Mutex<Vec<(String, bool)>>>,
    stop: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
}
impl Fixture {
    fn new(private: bool) -> Self {
        let listener = PORTS
            .iter()
            .find_map(|p| TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, *p)).ok())
            .expect("no permitted fixture port available");
        listener.set_nonblocking(true).unwrap();
        let url = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
        let tar = archive();
        let sri = format!(
            "sha512-{}",
            base64::engine::general_purpose::STANDARD.encode(sha2::Sha512::digest(&tar))
        );
        let path = if private {
            "/team/fixture.tgz"
        } else {
            "/foreign/fixture.tgz"
        };
        let metadata=serde_json::json!({"name":NAME,"dist-tags":{"latest":"1.0.0"},"versions":{"1.0.0":{"name":NAME,"version":"1.0.0","dist":{"tarball":format!("{url}{path}"),"integrity":sri}}},"time":{"1.0.0":"2020-01-01T00:00:00.000Z"}}).to_string().into_bytes();
        let seen = Arc::new(Mutex::new(Vec::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let observations = seen.clone();
        let stopping = stop.clone();
        let thread = std::thread::spawn(move || {
            while !stopping.load(Ordering::Relaxed) {
                let (mut stream, _) = match listener.accept() {
                    Ok(v) => v,
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(POLL_INTERVAL);
                        continue;
                    }
                    Err(e) => panic!("fixture accept failed: {e}"),
                };
                stream.set_read_timeout(Some(IO_TIMEOUT)).unwrap();
                stream.set_write_timeout(Some(IO_TIMEOUT)).unwrap();
                let mut request = Vec::new();
                let mut buf = [0; 1024];
                while !request.windows(4).any(|v| v == b"\r\n\r\n") && request.len() < MAX_HEADERS {
                    match stream.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => request.extend_from_slice(&buf[..n]),
                        Err(_) => break,
                    }
                }
                let text = String::from_utf8_lossy(&request);
                let target = text
                    .lines()
                    .next()
                    .and_then(|s| s.split_whitespace().nth(1))
                    .unwrap_or("")
                    .to_string();
                // An idle connection closed without an HTTP request is not a metadata fetch.
                // Chỉ ghi request thật; socket đóng khi chưa gửi HTTP không phải probe auth.
                if target.is_empty() {
                    continue;
                }
                let auth = text.lines().any(|line| {
                    line.eq_ignore_ascii_case("Authorization: Bearer synthetic-scope-fixture")
                });
                observations.lock().unwrap().push((target.clone(), auth));
                let artifact = target.ends_with(".tgz");
                let status = if artifact || auth {
                    "200 OK"
                } else {
                    "401 Unauthorized"
                };
                let body = if artifact { &tar } else { &metadata };
                let header = format!(
                    "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                let _ = stream.write_all(header.as_bytes());
                let _ = stream.write_all(body);
            }
        });
        Self {
            url,
            seen,
            stop,
            thread: Some(thread),
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            thread.join().expect("fixture thread failed")
        }
    }
}
#[test]
fn scoped_cli_install_v3_v4_web_lib_and_cloud() {
    for schema in ["3", "v4"] {
        for (core, route, kind) in [
            ("web", "install", ""),
            ("web", "install-web", ""),
            ("lib", "install", ""),
            #[cfg(feature = "clo")]
            ("clo", "install", "cdk"),
            #[cfg(feature = "clo")]
            ("clo", "install", "pulumi"),
        ] {
            // The fixture is the registry each route talks to, but only the
            // web and lib install paths read `.npmrc` credentials for it. The
            // cloud routes resolve through their own engine, and a fixture
            // named `scope-fixture` is not a real Terraform/CloudFormation
            // provider, so they cannot exercise npm credential scoping at all.
            // (Fixture là registry mà mỗi route nói tới, nhưng chỉ install path
            // của web và lib đọc credential `.npmrc` cho nó. Route cloud
            // resolve bằng engine riêng, và fixture `scope-fixture` không phải
            // provider Terraform/CloudFormation thật, nên không thể kiểm scope
            // credential npm.)
            if core == "clo" {
                continue;
            }
            for private in [true, false] {
                for nested in [false, true] {
                    let fixture = Fixture::new(private);
                    let sandbox = tempfile::tempdir().unwrap();
                    let project = sandbox.path().join("project");
                    let home = sandbox.path().join("home");
                    let temp = sandbox.path().join("tmp");
                    for p in [&project, &home, &temp] {
                        std::fs::create_dir_all(p).unwrap()
                    }
                    std::fs::write(project.join("mgc.toml"),format!("name=\"scope-project\"\nversion=\"1.0.0\"\necosystem=\"{core}\"\n[lock]\nschema=\"{schema}\"\n{}",match core { "lib" => "[lib]\nlanguage=\"ts\"\n".to_string(), "clo" => format!("[cloud]\ntype=\"{kind}\"\n"), _ => String::new() })).unwrap();
                    std::fs::write(project.join("package.json"),serde_json::json!({"name":"scope-project","version":"1.0.0","dependencies":{(NAME):"1.0.0"}}).to_string()).unwrap();
                    std::fs::write(project.join(".mgc.core"), format!("{core}\n")).unwrap();
                    std::fs::write(
                        project.join(".npmrc"),
                        format!(
                            "registry={}/team/\n{}/team/:_authToken=synthetic-scope-fixture\n",
                            fixture.url,
                            fixture.url.trim_start_matches("http:")
                        ),
                    )
                    .unwrap();
                    let mut cmd = std::process::Command::new(env!("CARGO_BIN_EXE_mgc"));
                    let cwd = if nested {
                        project.join("child")
                    } else {
                        project.clone()
                    };
                    std::fs::create_dir_all(&cwd).unwrap();
                    cmd.args([route, "--ignore-scripts"]).current_dir(&cwd);
                    for key in [
                        "MAGICORE_WEB_REGISTRY_URL",
                        "MAGICORE_WEB_REGISTRY_TOKEN",
                        "MAGICORE_WEB_ALLOWED_REGISTRIES",
                        "MAGICORE_LIB_REGISTRY_URL",
                        "MAGICORE_LIB_REGISTRY_TOKEN",
                        "MAGICORE_STRICT_INTEGRITY",
                    ] {
                        cmd.env_remove(key);
                    }
                    cmd.env("HOME", &home)
                        .env("USERPROFILE", &home)
                        .env("TMPDIR", &temp)
                        .env("TMP", &temp)
                        .env("TEMP", &temp)
                        .env("CI", "1")
                        .env("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST", "1")
                        .env("MGC_STRICT_INTEGRITY", "1");
                    for (key, dir) in [
                        ("MGC_DATA_DIR", "data"),
                        ("MGC_CACHE_DIR", "cache"),
                        ("MGC_STORE_DIR", "store"),
                    ] {
                        cmd.env(key, sandbox.path().join(dir));
                    }
                    let output = cmd.output().expect("launch real CLI");
                    assert!(
                        output.status.success(),
                        "{schema}/{core}/{route}/{private} failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                    let mut list_cmd = std::process::Command::new(cmd.get_program());
                    list_cmd.arg("list").current_dir(&cwd);
                    for (key, value) in cmd.get_envs() {
                        if let Some(value) = value {
                            list_cmd.env(key, value);
                        } else {
                            list_cmd.env_remove(key);
                        }
                    }
                    let listed = list_cmd
                        .output()
                        .expect("list installed packages through real CLI");
                    assert!(
                        listed.status.success(),
                        "{schema}/{core}/{route}/{private}/{nested} list failed: {}",
                        String::from_utf8_lossy(&listed.stderr)
                    );
                    assert!(
                        String::from_utf8_lossy(&listed.stdout).contains(NAME),
                        "list must include the materialized dependency"
                    );
                    if schema == "v4" {
                        let lock_path = project.join("mgc.lock");
                        let original = std::fs::read_to_string(&lock_path).unwrap();
                        let tampered =
                            original.replace("generator = \"mgc/", "generator = \"tampered/");
                        assert_ne!(
                            original, tampered,
                            "fixture must change authenticated lock material"
                        );
                        std::fs::write(&lock_path, tampered).unwrap();
                        let rejected = list_cmd.output().expect("list tampered V4 lock");
                        assert!(
                            !rejected.status.success(),
                            "list must reject a tampered V4 digest"
                        );
                        std::fs::write(&lock_path, original).unwrap();
                    }
                    let seen = fixture.seen.lock().unwrap();
                    let metadata: Vec<_> =
                        seen.iter().filter(|(p, _)| !p.ends_with(".tgz")).collect();
                    let artifacts: Vec<_> =
                        seen.iter().filter(|(p, _)| p.ends_with(".tgz")).collect();
                    assert!(
                        !metadata.is_empty() && metadata.iter().all(|(_, auth)| *auth),
                        "private metadata must authenticate: {schema}/{core}/{route}/{private}, requests={seen:?}, stdout={}, stderr={}",
                        String::from_utf8_lossy(&output.stdout),
                        String::from_utf8_lossy(&output.stderr)
                    );
                    assert!(
                        !artifacts.is_empty() && artifacts.iter().all(|(_, auth)| *auth == private),
                        "artifact authorization must match path scope"
                    );
                    assert!(
                        project
                            .join("node_modules")
                            .join(NAME)
                            .join("index.js")
                            .is_file()
                    );
                    let lock: toml::Value =
                        toml::from_str(&std::fs::read_to_string(project.join("mgc.lock")).unwrap())
                            .unwrap();
                    assert_eq!(
                        lock["version"].as_str(),
                        Some(if schema == "v4" { "4.1" } else { "3" })
                    );
                }
            }
        }
    }
}

#[test]
fn unused_npmrc_does_not_block_native_context_but_js_still_checks_it() {
    for (core, filename, manifest, expect_success) in [
        (
            "web",
            "pyproject.toml",
            "[project]\nname=\"fixture\"\nversion=\"1.0.0\"\n",
            true,
        ),
        (
            "web",
            "go.mod",
            "module example.test/fixture\ngo 1.23\n",
            true,
        ),
        (
            "lib",
            "pyproject.toml",
            "[project]\nname=\"fixture\"\nversion=\"1.0.0\"\n",
            true,
        ),
        (
            "web",
            "package.json",
            "{\"name\":\"fixture\",\"version\":\"1.0.0\"}",
            false,
        ),
    ] {
        let sandbox = tempfile::tempdir().unwrap();
        let root = sandbox.path().join("project");
        let home = sandbox.path().join("home");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::create_dir_all(&home).unwrap();
        std::fs::write(root.join("mgc.toml"), format!("name=\"fixture\"\nversion=\"1.0.0\"\necosystem=\"{core}\"\n[scripts]\nprobe=\"python3 --version\"\n")).unwrap();
        std::fs::write(root.join(".mgc.core"), format!("{core}\n")).unwrap();
        std::fs::write(root.join(filename), manifest).unwrap();
        std::fs::write(root.join(".npmrc"), "registry=not-a-url\n").unwrap();
        let mut cmd = std::process::Command::new(env!("CARGO_BIN_EXE_mgc"));
        cmd.args(["run", "probe"])
            .current_dir(&root)
            .env("HOME", &home)
            .env("USERPROFILE", &home);
        for name in [
            "MAGICORE_WEB_REGISTRY_URL",
            "MAGICORE_WEB_REGISTRY_TOKEN",
            "MAGICORE_WEB_ALLOWED_REGISTRIES",
        ] {
            cmd.env_remove(name);
        }
        for (name, directory) in [
            ("MGC_DATA_DIR", "data"),
            ("MGC_CACHE_DIR", "cache"),
            ("MGC_STORE_DIR", "store"),
        ] {
            cmd.env(name, sandbox.path().join(directory));
        }
        let output = cmd.output().expect("run context-isolation CLI probe");
        assert_eq!(
            output.status.success(),
            expect_success,
            "{core}/{filename} context outcome differs: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        if expect_success {
            assert!(String::from_utf8_lossy(&output.stdout).contains("Python"));
        } else {
            assert!(String::from_utf8_lossy(&output.stderr).contains("invalid registry URL"));
        }
    }
}
