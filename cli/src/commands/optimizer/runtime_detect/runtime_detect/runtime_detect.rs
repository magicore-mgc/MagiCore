//! Core dispatch and optimizer-specific runtime aggregation.
use super::super::{
    DetectedRuntime, detect_ai_runtime, detect_app_runtime, detect_cloud_node_runtime,
    detect_lib_runtime, detect_package_manager, detect_web_runtime, project_text,
};
use std::path::Path;

/// Detect runtime(s) for a project at given path — phát hiện runtime cho project tại đường dẫn
/// Multiple runtimes can coexist (e.g., monorepo with web + ai) — nhiều runtime có thể cùng tồn tại
pub fn detect_runtimes(project_root: &Path, core: &str) -> Vec<DetectedRuntime> {
    let mut runtimes = vec![];

    match core {
        "web" => {
            runtimes.extend(detect_web_runtime(project_root));
        }
        "ai" => {
            runtimes.extend(detect_ai_runtime(project_root));
        }
        "lib" => {
            runtimes.extend(detect_lib_runtime(project_root));
        }
        "app" => {
            runtimes.extend(detect_app_runtime(project_root));
        }
        "clo" | "cloud" if detect_cloud_node_runtime(project_root) => {
            // Cloud IaC projects use Node.js for Pulumi/CDK manifests; keep the
            // runtime identity separate from the package-manager choice.
            // (Pulumi/CDK chạy trên Node.js; phân biệt runtime với package manager.)
            runtimes.push(DetectedRuntime::NodeJs {
                package_manager: detect_package_manager(project_root),
            });
        }
        _ => {
            // Game/iot/cloud/cicd — generic detection or fallback — phát hiện chung hoặc dự phòng
            if project_root.join("Cargo.toml").exists() {
                runtimes.push(DetectedRuntime::RustLib);
            }
        }
    }

    if runtimes.is_empty() {
        runtimes.push(DetectedRuntime::Unknown);
    }

    runtimes
}

/// Add explicit web-language runtimes for optimizer dispatch without changing
/// the narrower runtime contract consumed by `mgc test`.
/// (Thêm runtime web theo manifest cho optimizer, không đổi hợp đồng của `mgc test`.)
pub fn detect_optimizer_runtimes(project_root: &Path, core: &str) -> Vec<DetectedRuntime> {
    let mut runtimes = detect_runtimes(project_root, core);
    if core == "web" {
        runtimes.retain(|runtime| *runtime != DetectedRuntime::Unknown);

        let cargo_package = project_text(project_root, "Cargo.toml")
            .and_then(|content| toml::from_str::<toml::Value>(&content).ok())
            .is_some_and(|manifest| {
                manifest
                    .get("package")
                    .and_then(|package| package.get("name"))
                    .and_then(toml::Value::as_str)
                    .is_some_and(|name| !name.trim().is_empty())
            });
        if cargo_package {
            runtimes.push(DetectedRuntime::RustLib);
        }

        let go_module = project_text(project_root, "go.mod").is_some_and(|content| {
            content.lines().any(|line| {
                let mut fields = line.split_whitespace();
                fields.next() == Some("module") && fields.next().is_some()
            })
        });
        if go_module {
            runtimes.push(DetectedRuntime::GoLib);
        }

        let python_project = project_text(project_root, "pyproject.toml")
            .and_then(|content| toml::from_str::<toml::Value>(&content).ok())
            .is_some_and(|manifest| {
                manifest
                    .get("project")
                    .and_then(|project| project.get("name"))
                    .and_then(toml::Value::as_str)
                    .is_some_and(|name| !name.trim().is_empty())
                    || manifest
                        .get("tool")
                        .and_then(|tool| tool.get("poetry"))
                        .and_then(|poetry| poetry.get("name"))
                        .and_then(toml::Value::as_str)
                        .is_some_and(|name| !name.trim().is_empty())
            })
            || project_text(project_root, "setup.py").is_some_and(|content| {
                content.lines().any(|line| {
                    let line = line.trim_start();
                    !line.starts_with('#')
                        && [
                            "setup(",
                            "setup (",
                            "setuptools.setup(",
                            "setuptools.setup (",
                        ]
                        .iter()
                        .any(|prefix| line.starts_with(prefix))
                })
            })
            || project_text(project_root, "requirements.txt").is_some_and(|content| {
                content.lines().any(|line| {
                    let requirement = line.split('#').next().unwrap_or_default().trim();
                    let is_nested_or_editable = requirement.starts_with("-r ")
                        || requirement.starts_with("-e ")
                        || requirement.starts_with("--requirement ")
                        || requirement.starts_with("--editable ");
                    !requirement.is_empty()
                        && (!requirement.starts_with('-') || is_nested_or_editable)
                })
            });
        if python_project {
            runtimes.push(DetectedRuntime::PythonLib);
        }
    }

    if runtimes.is_empty() {
        runtimes.push(DetectedRuntime::Unknown);
    }
    runtimes
}
