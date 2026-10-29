//! Argument assembly for each `DestinationArg` shape.
//! Lắp tham số cho từng dạng `DestinationArg`.
//!
//! The initializer catalog previously appended the destination path to every
//! command, which cannot work for tools that take a project NAME (`cargo new`,
//! `dotnet new`) or run inside the directory (`go mod init`, `uv init`). These
//! tests pin the command line each shape produces so a future record cannot
//! silently reintroduce the broken form.
//! (Catalog initializer từng nối đường dẫn đích vào mọi lệnh, không thể chạy với
//! tool nhận TÊN project (`cargo new`, `dotnet new`) hoặc chạy trong thư mục
//! (`go mod init`, `uv init`). Các test dưới đây ghim dòng lệnh từng dạng sinh
//! ra, để record thêm sau không lặng lẽ mang lại dạng hỏng.)

use std::path::Path;

use crate::scaffold::core_initializers::types::DestinationArg;
use crate::scaffold::core_initializers::RECORDS;

/// The catalog must describe each framework with the shape its tool accepts.
/// (Catalog phải mô tả mỗi framework bằng dạng mà tool của nó chấp nhận.)
#[test]
fn catalog_uses_the_destination_shape_each_tool_accepts() {
    // `cargo new --lib <path>` and `flutter create <path>` take a positional path.
    for framework in ["rust", "java", "flutter"] {
        let record = lookup(&["lib", "app"], framework)
            .unwrap_or_else(|| panic!("{framework} must be registered"));
        assert_eq!(
            record.destination,
            DestinationArg::Positional,
            "{framework} takes a positional destination"
        );
    }

    // Tools that run inside the target directory and only name the project.
    for (ecosystem, framework) in [
        ("lib", "go"),
        ("lib", "python"),
        ("ai", "python-agent"),
        ("ai", "mcp-server"),
        ("clo", "cdk-typescript"),
        ("iot", "zephyr"),
    ] {
        let record = lookup(&[ecosystem], framework)
            .unwrap_or_else(|| panic!("{ecosystem}/{framework} must be registered"));
        assert_eq!(
            record.destination,
            DestinationArg::NameInCwd,
            "{ecosystem}/{framework} runs inside the destination"
        );
    }

    // Tools that take the path through a named flag.
    for (ecosystem, framework) in [
        ("lib", "dotnet"),
        ("game", "godot"),
        ("iot", "platformio"),
        ("clo", "terraform"),
        ("clo", "pulumi"),
    ] {
        let record = lookup(&[ecosystem], framework)
            .unwrap_or_else(|| panic!("{ecosystem}/{framework} must be registered"));
        assert!(
            matches!(record.destination, DestinationArg::Flag(_)),
            "{ecosystem}/{framework} takes the path through a flag"
        );
    }
}

/// A destination directory always yields a usable project name.
/// (Thư mục đích luôn cho ra tên project dùng được.)
#[test]
fn project_name_comes_from_the_final_path_component() {
    assert_eq!(
        super::super::project_name(Path::new("/tmp/out/my-lib")).expect("name"),
        "my-lib"
    );
}

/// A pathological path fails with guidance rather than an empty name.
/// (Path bất thường báo lỗi kèm hướng dẫn thay vì tên rỗng.)
#[test]
fn a_path_without_a_final_component_is_rejected() {
    let error = super::super::project_name(Path::new("/"))
        .expect_err("root has no project name");
    let text = error.to_string();
    assert!(
        text.contains("cannot derive a project name"),
        "unexpected message: {text}"
    );
}

/// The catalog record for a framework in any of the given ecosystems.
/// (Record catalog cho framework trong một trong các ecosystem đã cho.)
fn lookup(ecosystems: &[&str], framework: &str) -> Option<&'static crate::scaffold::core_initializers::types::CoreInitializer> {
    RECORDS
        .iter()
        .find(|record| ecosystems.contains(&record.ecosystem) && record.framework == framework)
}
