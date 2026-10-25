//! Command-line initializers for the non-Web cores.
//! Initializer CLI cho các core không phải Web.
//!
//! Every entry names a program the framework's own project publishes. MagiCore
//! adds no files; it runs the program and checks that the documented marker
//! appeared, so a program that silently does nothing cannot look like a success.
//! Mỗi entry nêu chương trình do chính framework phát hành. MagiCore không thêm
//! file; nó chạy chương trình và kiểm marker đã xuất hiện, nên một chương trình
//! chạy mà không làm gì không thể trông giống thành công.

use super::super::types::CoreInitializer;

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
    },
    CoreInitializer {
        ecosystem: "iot",
        framework: "platformio",
        program: "pio",
        args_before: &["project", "init", "--project-dir"],
        args_after: &["--board", "esp32dev"],
        probe: &["--version"],
        install_hint: "pip install platformio",
        marker: "platformio.ini",
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
    },
];

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
