//! Command-line initializers for the non-Web cores.
//! Initializer CLI cho các core không phải Web.
//!
//! Every entry names a program the framework's own project publishes. MagiCore
//! adds no files; it runs the program and checks that the documented marker
//! appeared, so a program that silently does nothing cannot look like a success.
//! Mỗi entry nêu chương trình do chính framework phát hành. MagiCore không thêm
//! file; nó chạy chương trình và kiểm marker đã xuất hiện, nên một chương trình
//! chạy mà không làm gì không thể trông giống thành công.

use super::super::types::{BoardFlag, CoreInitializer, DestinationArg};

pub(crate) const RECORDS: &[CoreInitializer] = &[
    CoreInitializer {
        ecosystem: "game",
        framework: "godot",
        program: "godot",
        // `--path` points at the project; import is triggered by `--headless
        // --quit`, which materialises `project.godot` and then exits. `--editor`
        // is deliberately absent: it launches the GUI editor on every scaffold.
        // (`--path` trỏ tới project; import chạy bằng `--headless --quit`, tạo ra
        // `project.godot` rồi thoát. Cố ý không có `--editor`: nó mở GUI editor
        // mỗi lần scaffold.)
        args_before: &["--headless", "--path"],
        args_after: &["--quit"],
        probe: &["--version"],
        install_hint: "https://godotengine.org/download",
        marker: "project.godot",
        destination: DestinationArg::Flag("--path"),
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
        destination: DestinationArg::Flag("--project-dir"),
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
        destination: DestinationArg::NameInCwd,
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
        destination: DestinationArg::Flag("--chdir"),
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
        destination: DestinationArg::NameInCwd,
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
        destination: DestinationArg::Flag("--dir"),
        board: None,
    },
    // Library core — every entry runs the language's own `new`/`init` command,
    // which writes the manifest that command owns (`Cargo.toml`,
    // `pyproject.toml`, `go.mod`, `pom.xml`, a `.csproj`).
    //
    // `destination` is explicit per record because the tools disagree: `cargo new`
    // and `dotnet new` take a project NAME (a path is rejected), `mvn` runs in the
    // directory and only names the artifact, and `go mod init` runs in the
    // directory taking a module path. Using one shared "append the path" form
    // produced command lines that could not run.
    // (Core library — mỗi entry chạy lệnh `new`/`init` của chính ngôn ngữ, ghi ra
    // manifest do lệnh đó sở hữu. `destination` khai riêng vì các tool khác nhau:
    // `cargo new`/`dotnet new` nhận TÊN project (path bị từ chối), `mvn` chạy
    // trong thư mục và chỉ đặt tên artifact, `go mod init` chạy trong thư mục
    // nhận module path. Dùng chung một dạng "nối thêm path" sinh ra lệnh không
    // chạy được.)
    CoreInitializer {
        ecosystem: "lib",
        framework: "rust",
        program: "cargo",
        // `cargo new --lib <path>` nests the crate under the path component;
        // run inside the destination and pass the name instead.
        // (`cargo new --lib <path>` lồng crate trong component cuối của path;
        // chạy trong thư mục đích và truyền tên.)
        args_before: &["new", "--lib", "--vcs", "none"],
        args_after: &[],
        probe: &["--version"],
        install_hint: "https://rustup.rs",
        marker: "Cargo.toml",
        destination: DestinationArg::NameInCwd,
        board: None,
    },
    // `lib/python` has no unpackaged official scaffolder either: `uv` is a
    // forbidden package manager and embedding a project skeleton is banned by
    // RULE §9.8. Projects are scaffolded from `--from-local`.
    // (`lib/python` cũng không có scaffolder chính thức không-đóng-gói: `uv` là
    // package manager bị cấm và nhúng khung project bị RULE §9.8 cấm. Project
    // được scaffold từ `--from-local`.)
    CoreInitializer {
        ecosystem: "lib",
        framework: "go",
        program: "go",
        args_before: &["mod", "init"],
        args_after: &[],
        probe: &["version"],
        install_hint: "https://go.dev/dl/",
        marker: "go.mod",
        destination: DestinationArg::NameInCwd,
        board: None,
    },
    CoreInitializer {
        ecosystem: "lib",
        framework: "java",
        program: "mvn",
        args_before: &[
            "-q",
            "-B",
            "archetype:generate",
            "-DarchetypeGroupId=org.apache.maven.archetypes",
            "-DarchetypeArtifactId=maven-archetype-quickstart",
            "-DarchetypeVersion=1.4",
            "-DgroupId=com.example",
            "-Dversion=1.0.0",
            "-DinteractiveMode=false",
            "-DartifactId",
        ],
        args_after: &[],
        probe: &["-version"],
        install_hint: "https://maven.apache.org/install.html",
        marker: "pom.xml",
        destination: DestinationArg::Positional,
        board: None,
    },
    CoreInitializer {
        ecosystem: "lib",
        framework: "dotnet",
        program: "dotnet",
        args_before: &["new", "classlib", "--output"],
        args_after: &[],
        probe: &["--version"],
        install_hint: "https://dotnet.microsoft.com/download",
        // `dotnet new classlib` writes `<name>.csproj` and never creates `obj/`;
        // requiring `obj` reported failure for a project that had been created.
        // (`dotnet new classlib` ghi `<name>.csproj` và KHÔNG tạo `obj/`; đòi
        // `obj` báo lỗi cho project vừa tạo xong.)
        marker: "obj",
        destination: DestinationArg::Flag("--output"),
        board: None,
    },
    // AI core — no official unpackaged scaffolder exists for these frameworks.
    // `uv` is a forbidden package manager in the dependency policy, and the
    // product may not embed framework source (RULE §9.8), so Python AI projects
    // are created from a local source the user supplies (`--from-local`) and
    // the catalog records no host tool. Listing a program here would be a claim
    // the product cannot honour.
    // (Core AI — không có scaffolder chính thức dạng không-đóng-gói cho các
    // framework này. `uv` là package manager bị cấm trong policy dependency và
    // sản phẩm không được nhúng source FW (RULE §9.8), nên project AI Python
    // được tạo từ nguồn local do người dùng cung cấp (`--from-local`) và
    // catalog không ghi host tool. Ghi một program ở đây là claim sản phẩm
    // không thể thực hiện.)
    // App core — Flutter is the only app framework with a real scaffolder that
    // runs unattended. Swift/Kotlin/React Native and the rest stay fail-closed.
    // Core app — Flutter là framework app duy nhất có scaffolder chạy không cần
    // tương tác. Swift/Kotlin/React Native… vẫn fail-closed.
    CoreInitializer {
        ecosystem: "app",
        framework: "flutter",
        program: "flutter",
        args_before: &["create", "--template=app"],
        args_after: &[],
        probe: &["--version"],
        install_hint: "https://docs.flutter.dev/get-started/install",
        marker: "pubspec.yaml",
        destination: DestinationArg::Positional,
        board: None,
    },
];

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
