//! Project detection and core selection diagnostics.
//! Lỗi nhận diện project và chọn core.

use anyhow::Error;
use anyhow::anyhow;

/// cwd đã bị xóa khi resolve
pub fn cwd_deleted(e: &std::io::Error) -> Error {
    anyhow!("failed to resolve current working directory — has it been deleted?: {e}")
}

/// không detect được project theo core kind
pub fn no_mgc_project_found(kind: &str) -> Error {
    let msg = match kind {
        "game" => {
            "No MagiCore game project found (missing mgc.toml with ecosystem = \"game\" \
                   or project.godot/Packages/manifest.json/.uproject/Cargo.toml in the current project)"
        }
        "iot" => {
            "No MagiCore IoT project found (missing mgc.toml with ecosystem = \"iot\" \
                  or platformio.ini/west.yml/Cargo.toml in the current project)"
        }
        "lib" | "library" => {
            "No MagiCore library project found (missing mgc.toml with \
                  ecosystem = \"lib\" or Cargo.toml/package.json/pyproject.toml in the current project)"
        }
        "clo" | "cloud" => {
            "No MagiCore cloud project found (missing mgc.toml with ecosystem = \"cloud\" \
                  or Pulumi.yaml/*.tf/cdk package.json in the current project)"
        }
        "app" => {
            "No MagiCore app project found (missing mgc.toml with ecosystem = \"app\" \
                  or pubspec.yaml/build.gradle/.kts/Package.swift in the current project)"
        }
        "web" => {
            "No MagiCore project found (current directory and its parents contain no mgc.toml, mgc.lock, package.json, Cargo.toml, pyproject.toml, or .git)"
        }
        _ => {
            "No MagiCore project found (missing mgc.toml or known project manifest in the current directory tree)"
        }
    };
    anyhow!(msg)
}

/// router core không biết core
pub fn unknown_core(core: &str) -> Error {
    anyhow!("Unknown core '{core}'")
}

/// bare package command needs an explicit or detected core
pub fn bare_core_not_detected(verb: &str) -> Error {
    anyhow!(
        "`mgc {verb}` could not detect a MagiCore core in this project. Use `mgc --core <core> {verb}` or the explicit command such as `mgc {verb}-web`."
    )
}

/// chưa chạy mgc init
pub fn project_root_missing() -> Error {
    anyhow!("Project root not found — run mgc init")
}

/// thiếu mgc.toml
pub fn mgc_toml_missing() -> Error {
    anyhow!("mgc.toml not found — run mgc init")
}

/// không detect framework theo loại
pub fn no_framework_detected(framework: &str, root: &std::path::Path) -> Error {
    anyhow!("No {framework} framework detected in {}", root.display())
}

/// tool chạy fail (generic)
pub fn tool_failed(tool: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("{tool} failed: {e}")
}

/// vượt MAX_PACKAGES
pub fn too_many_packages(count: usize, verb: &str) -> Error {
    anyhow!("Too many packages ({count}). Maximum per {verb} command is 50.")
}

/// package không thấy local/global
pub fn pkg_not_found_local(package: &str) -> Error {
    anyhow!(
        "package '{package}' not found in local node_modules or MagiCore global package roots.\n\
         Use 'mgc install' to install it first, provide a local path, or configure MAGICORE_GLOBAL_PACKAGE_ROOT."
    )
}

pub fn workspace_failed(count: usize) -> Error {
    anyhow!("{count} workspace(s) failed to install")
}

pub fn unknown_ecosystem(ecosystem: &str) -> Error {
    anyhow!("Unknown ecosystem: '{ecosystem}'")
}

pub fn no_registry_configured() -> Error {
    anyhow!("no registry configured")
}

pub fn cannot_detect_project_type(root: &std::path::Path) -> Error {
    anyhow!(
        "Cannot detect project type in '{}'. Run `mgc init --template <type>`, \
         `mgc init --signature <core>` to write a plain-text core marker, or specify `--core <type>`",
        root.display()
    )
}

pub fn no_mgc_project_root() -> Error {
    anyhow!("No MagiCore project found. Run `mgc init` to create one, or specify `--core <type>`.")
}

pub fn core_not_in_build(core: &str) -> Error {
    anyhow!("'{core}' core is not included in this build.")
}

pub fn detect_core_failed(kind: &str) -> Error {
    let msg = match kind {
        "game" => {
            "Cannot detect a game project here (missing mgc.toml/project.godot/manifest.json/.uproject)."
        }
        "ai" => {
            "Cannot detect an ai project here (missing mgc.toml [ai] framework / pyproject [tool.magicore] framework)."
        }
        "clo" | "cloud" => {
            "Cannot detect a cloud project here (missing mgc.toml/Pulumi.yaml/*.tf/cdk package.json)."
        }
        "cicd" => {
            "Cannot detect a cicd project here (missing mgc.toml/wrangler.toml/argocd/.github/workflows)."
        }
        "iot" => "Cannot detect an iot project here (missing mgc.toml/platformio.ini/west.yml).",
        "app" => {
            "Cannot detect an app project here (missing mgc.toml/pubspec.yaml/build.gradle/Package.swift)."
        }
        "hardware" => {
            "Cannot detect a hardware project here (missing mgc.toml with ecosystem = \"hardware\")."
        }
        "lib" => "Cannot detect a lib project here (missing mgc.toml/lib marker).",
        _ => "Cannot detect a project here.",
    };
    anyhow!(msg)
}
