//! Command-line initializers for the non-Web cores.
//! Initializer CLI cho các core không phải Web.
//!
//! Every entry names a program the framework's own project publishes. MagiCore
//! adds no files; it runs the program and checks that the documented marker
//! appeared, so a program that silently does nothing cannot look like a success.
//! Mỗi entry nêu chương trình do chính framework phát hành. MagiCore không thêm
//! file; nó chạy chương trình và kiểm marker đã xuất hiện, nên một chương trình
//! chạy mà không làm gì không thể trông giống thành công.

use super::super::types::{BoardFlag, CoreInitializer};

pub(crate) const RECORDS: &[CoreInitializer] = &[
    CoreInitializer {
        ecosystem: "game",
        framework: "godot",
        program: "godot",
        // `--path` points at the project; `--quit` exits after importing, which
        // is what materialises `project.godot`.
        // `--path` trỏ tới project; `--quit` thoát sau khi import, chính bước
        // đó tạo ra `project.godot`.
        args_before: &["--headless", "--path"],
        args_after: &["--quit", "--editor"],
        probe: &["--version"],
        install_hint: "https://godotengine.org/download",
        marker: "project.godot",
        board: None,
    },
    CoreInitializer {
        ecosystem: "iot",
        framework: "platformio",
        program: "pio",
        args_before: &["project", "init", "--project-dir"],
        args_after: &[],
        probe: &["--version"],
        install_hint: "pip install platformio",
        marker: "platformio.ini",
        board: Some(BoardFlag {
            flag: "--board",
            default: "esp32dev",
        }),
    },
    CoreInitializer {
        ecosystem: "iot",
        framework: "zephyr",
        program: "west",
        args_before: &["init"],
        args_after: &[],
        probe: &["--version"],
        install_hint: "pip install west",
        marker: "west.yml",
        board: None,
    },
    CoreInitializer {
        ecosystem: "clo",
        framework: "terraform",
        program: "terraform",
        args_before: &["init"],
        args_after: &[],
        probe: &["version"],
        install_hint: "https://developer.hashicorp.com/terraform/downloads",
        marker: ".terraform",
        board: None,
    },
    CoreInitializer {
        ecosystem: "clo",
        framework: "cdk-typescript",
        program: "cdk",
        args_before: &["init", "--language", "typescript", "--app"],
        args_after: &[],
        probe: &["--version"],
        install_hint: "npm install -g aws-cdk",
        marker: "cdk.json",
        board: None,
    },
    CoreInitializer {
        ecosystem: "clo",
        framework: "pulumi",
        program: "pulumi",
        args_before: &["new", "typescript", "--yes", "--dir"],
        args_after: &[],
        probe: &["version"],
        install_hint: "https://www.pulumi.com/docs/install/",
        marker: "Pulumi.yaml",
        board: None,
    },
    // Library core — every entry runs the language's own `new`/`init` command,
    // which writes the manifest that command owns (`Cargo.toml`,
    // `pyproject.toml`, `go.mod`, `pom.xml`, a `.csproj`).
    // Core library — mỗi entry chạy lệnh `new`/`init` của chính ngôn ngữ, ghi ra
    // manifest do lệnh đó sở hữu.
    CoreInitializer {
        ecosystem: "lib",
        framework: "rust",
        program: "cargo",
        args_before: &["new", "--lib", "--vcs", "none", "--name"],
        args_after: &[],
        probe: &["--version"],
        install_hint: "https://rustup.rs",
        marker: "Cargo.toml",
        board: None,
    },
    CoreInitializer {
        ecosystem: "lib",
        framework: "python",
        program: "uv",
        // `uv init --lib` writes `pyproject.toml` plus the package directory;
        // plain `pip` has no scaffolding command at all.
        // `uv init --lib` ghi `pyproject.toml` và thư mục package; `pip` không có
        // lệnh scaffold nào.
        args_before: &["init", "--lib", "--name"],
        args_after: &[],
        probe: &["--version"],
        install_hint: "https://docs.astral.sh/uv/getting-started/installation/",
        marker: "pyproject.toml",
        board: None,
    },
    CoreInitializer {
        ecosystem: "lib",
        framework: "go",
        program: "go",
        // `go mod init` needs a module path, not a directory; the caller supplies
        // the project name and Go derives the directory from the working copy.
        // `go mod init` cần module path; caller cấp tên project.
        args_before: &["mod", "init"],
        args_after: &[],
        probe: &["version"],
        install_hint: "https://go.dev/dl/",
        marker: "go.mod",
        board: None,
    },
    CoreInitializer {
        ecosystem: "lib",
        framework: "java",
        program: "mvn",
        args_before: &[
            "-q",
            "archetype:generate",
            "-DartifactId",
        ],
        args_after: &[
            "-DarchetypeGroupId=org.apache.maven.archetypes",
            "-DarchetypeArtifactId=maven-archetype-quickstart",
            "-DgroupId=com.example",
            "-DinteractiveMode=false",
        ],
        probe: &["-version"],
        install_hint: "https://maven.apache.org/install.html",
        marker: "pom.xml",
        board: None,
    },
    CoreInitializer {
        ecosystem: "lib",
        framework: "dotnet",
        program: "dotnet",
        args_before: &["new", "classlib", "--name"],
        args_after: &[],
        probe: &["--version"],
        install_hint: "https://dotnet.microsoft.com/download",
        marker: "obj",
        board: None,
    },
    // AI core — the two frameworks the registry advertises, each with its own
    // official scaffolder.
    // Core AI — hai framework registry khai báo, mỗi cái có scaffolder chính thức.
    CoreInitializer {
        ecosystem: "ai",
        framework: "python-agent",
        program: "uv",
        args_before: &["init", "--name"],
        args_after: &[],
        probe: &["--version"],
        install_hint: "https://docs.astral.sh/uv/getting-started/installation/",
        marker: "pyproject.toml",
        board: None,
    },
    CoreInitializer {
        ecosystem: "ai",
        framework: "mcp-server",
        program: "uv",
        args_before: &["init", "--name"],
        args_after: &[],
        probe: &["--version"],
        install_hint: "https://docs.astral.sh/uv/getting-started/installation/",
        marker: "pyproject.toml",
        board: None,
    },
    // App core — Flutter is the only app framework with a real scaffolder that
    // runs unattended. Swift/Kotlin/React Native and the rest stay fail-closed.
    // Core app — Flutter là framework app duy nhất có scaffolder chạy không cần
    // tương tác. Swift/Kotlin/React Native… vẫn fail-closed.
    CoreInitializer {
        ecosystem: "app",
        framework: "flutter",
        program: "flutter",
        args_before: &["create", "--template=app", "--project-name"],
        args_after: &[],
        probe: &["--version"],
        install_hint: "https://docs.flutter.dev/get-started/install",
        marker: "pubspec.yaml",
        board: None,
    },
];

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
