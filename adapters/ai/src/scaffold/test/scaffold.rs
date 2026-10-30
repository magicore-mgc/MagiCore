//! AI scaffold generator tests — kiểm thử generator scaffold AI.
//!
//! Every case writes into a fresh temporary directory and asserts the files each
//! framework owns. No network and no interpreter is started, so a green run
//! proves the offline generator works.
//! Mỗi ca ghi vào thư mục tạm mới và kiểm file từng framework sở hữu. Không
//! mạng, không khởi động interpreter — test xanh là bằng chứng generator offline
//! chạy được.

use std::path::{Path, PathBuf};

use crate::scaffold::layout::{self, pyproject_toml};
use crate::scaffold::{mcp_server, scaffold_project};

use crate::framework::AiFramework;

#[test]
fn python_agent_writes_a_detectable_python_package() {
    let target = temp_target("python-agent");
    run(AiFramework::PythonAgent, "python-agent", &target);

    for relative in [
        "pyproject.toml",
        "README.md",
        ".gitignore",
        "src/agent.py",
        "src/python_agent/__init__.py",
        "src/python_agent/agent.py",
        "src/python_agent/__main__.py",
    ] {
        assert!(target.join(relative).is_file(), "{relative} missing");
    }

    let pyproject = read(&target.join("pyproject.toml"));
    assert!(
        pyproject.contains("[tool.magicore]\nframework = \"python-agent\""),
        "pyproject must carry the framework marker:\n{pyproject}"
    );
    assert!(pyproject.contains("name = \"python-agent\""));
    assert!(pyproject.contains("\"openai\""));
    assert!(
        pyproject.contains("[project.scripts]"),
        "a runnable console script must be declared"
    );

    // The adapter's own detection is the contract the create lane relies on.
    // (Detection của chính adapter là contract mà create lane dựa vào.)
    assert_eq!(
        crate::framework::detect_framework(&target),
        Some(AiFramework::PythonAgent)
    );
    let shim = read(&target.join("src/agent.py"));
    assert!(
        shim.contains("from python_agent.agent import run"),
        "entry shim must import the generated package:\n{shim}"
    );

    cleanup(&target);
}

#[test]
fn mcp_server_writes_a_stdio_json_rpc_loop() {
    let target = temp_target("mcp-server");
    run(AiFramework::McpServer, "mcp-server", &target);

    for relative in [
        "pyproject.toml",
        "README.md",
        "server.py",
        ".gitignore",
        "src/mcp_server/__init__.py",
        "src/mcp_server/tools.py",
    ] {
        assert!(target.join(relative).is_file(), "{relative} missing");
    }

    let server = read(&target.join("server.py"));
    for expected in [
        "json.loads",
        "json.dumps",
        "\"initialize\"",
        "\"tools/list\"",
        "\"tools/call\"",
        "-32601",
    ] {
        assert!(
            server.contains(expected),
            "server.py must speak JSON-RPC ({expected} missing)"
        );
    }
    assert!(
        server.contains(&format!(
            "PROTOCOL_VERSION = \"{}\"",
            mcp_server::PROTOCOL_VERSION
        )),
        "server must advertise the pinned protocol revision"
    );
    assert!(
        !server.contains('@'),
        "placeholders must all be substituted:\n{server}"
    );
    assert!(server.contains("from mcp_server.tools import TOOLS, handler_for"));
    assert!(read(&target.join("src/mcp_server/tools.py")).contains("def echo("));

    assert_eq!(
        crate::framework::detect_framework(&target),
        Some(AiFramework::McpServer)
    );

    cleanup(&target);
}

#[test]
fn project_name_maps_to_a_valid_package_directory() {
    let target = temp_target("named");
    run(AiFramework::PythonAgent, "My Agent.App", &target);

    assert!(
        target.join("src/my_agent_app/agent.py").is_file(),
        "package directory must be a Python identifier"
    );
    let pyproject = read(&target.join("pyproject.toml"));
    assert!(pyproject.contains("name = \"my-agent-app\""));
    assert!(pyproject.contains("my-agent-app = \"my_agent_app.__main__:main\""));
    cleanup(&target);
}

#[test]
fn existing_target_fails_closed() {
    let target = temp_target("existing");
    std::fs::create_dir_all(&target).expect("create target");
    let outcome = runtime().block_on(scaffold_project(
        AiFramework::PythonAgent,
        "existing",
        &target,
    ));
    assert!(outcome.is_err(), "an existing destination must be refused");
    assert!(
        std::fs::read_dir(&target)
            .expect("read target")
            .next()
            .is_none(),
        "a refused create must leave the directory untouched"
    );
    cleanup(&target);
}

#[test]
fn symlinked_destination_is_refused() {
    let target = temp_target("symlink");
    std::fs::create_dir_all(&target).expect("create target");
    let outside = temp_target("outside");
    std::fs::write(&outside, "untouched").expect("seed outside file");
    std::os::unix::fs::symlink(&outside, target.join("README.md")).expect("link");

    let outcome = runtime().block_on(scaffold_project(AiFramework::McpServer, "symlink", &target));
    assert!(outcome.is_err(), "a symlinked destination must be refused");
    assert_eq!(read(&outside), "untouched");
    std::fs::remove_dir_all(&target).ok();
    std::fs::remove_file(&outside).ok();
}

#[test]
fn rendered_manifest_stays_inside_the_owned_dependency_subset() {
    // The PyPI ownership predicate only models `name` and `name>=version`
    // entries; an extra, marker, or URL would make the fresh project
    // unmanageable by MagiCore on its very first install.
    // (Predicate sở hữu PyPI chỉ mô hình `name` và `name>=version`; extras,
    // marker hay URL sẽ làm project mới không quản lý được ngay lần install
    // đầu tiên.)
    for engine in [AiFramework::PythonAgent, AiFramework::McpServer] {
        let target = temp_target("subset");
        let layout = layout::ProjectLayout::new(&target, "subset");
        let rendered = pyproject_toml(&layout, engine);
        let document: toml::Value = toml::from_str(&rendered).expect("valid toml");
        let dependencies = document["project"]["dependencies"]
            .as_array()
            .expect("dependencies array");
        assert!(!dependencies.is_empty());
        for dependency in dependencies {
            let spec = dependency.as_str().expect("dependency string");
            assert!(
                spec.chars()
                    .all(|character| character.is_ascii_alphanumeric()
                        || ".-=_<>".contains(character)),
                "unsupported dependency syntax '{spec}' for {engine:?}"
            );
        }
    }
}

/// Scaffold one framework into `target` and assert it wrote files.
/// Scaffold một framework vào `target` và đảm bảo đã ghi file.
fn run(engine: AiFramework, name: &str, target: &Path) {
    runtime()
        .block_on(scaffold_project(engine, name, target))
        .unwrap_or_else(|error| panic!("{engine:?} scaffold failed: {error}"));
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
}

/// A unique, not-yet-existing directory — thư mục duy nhất, chưa tồn tại.
fn temp_target(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "mgc-ai-scaffold-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ))
}

fn cleanup(dir: &Path) {
    std::fs::remove_dir_all(dir).ok();
}
