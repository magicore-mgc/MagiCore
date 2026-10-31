//! Resolves the start command of a split fullstack project's `server/` half.
//! Resolve lệnh khởi động nửa `server/` của project fullstack tách rời.
//!
//! `mgc dev` reads the frontend from the project manifests; the server half is
//! a separate tree with its own manifest (`package.json`, `pyproject.toml`,
//! `Cargo.toml`, `go.mod`, `manage.py`). Those names are the same ones
//! `manifest_presence` uses, so a directory is a backend half for both owners.
//! `mgc dev` đọc frontend từ manifest của project; nửa server là cây riêng với
//! manifest riêng (`package.json`, `pyproject.toml`, `Cargo.toml`, `go.mod`,
//! `manage.py`). Tên này trùng với `manifest_presence`, nên một thư mục là nửa
//! backend cho cả hai owner.

use std::path::Path;

/// Marker files that make a directory a backend half.
/// File marker khiến một thư mục là nửa backend.
///
/// Listed here rather than inline so the dev lane and the project-mode detector
/// are read against one name per ecosystem.
/// Liệt kê ở đây thay vì inline để lane dev và mode detector đọc cùng một tên cho
/// mỗi hệ sinh thái.
const BACKEND_MARKERS: &[&str] = &[
    "package.json",
    "go.mod",
    "Cargo.toml",
    "manage.py",
    "main.py",
    "app.py",
    "pyproject.toml",
    "src/main.py",
];

/// Script names `mgc dev` accepts from the server's `package.json`.
/// Tên script mà `mgc dev` nhận từ `package.json` của server.
///
/// `dev` first because that is what a generated starter exposes on a server
/// root; `dev:server`/`start` are the names starter layouts use for the API.
/// `dev` trước vì starter sinh ra phơi script đó ở gốc server; `dev:server`/
/// `start` là tên mà bố cục starter dùng cho API.
const BACKEND_SCRIPTS: &[&str] = &["dev", "dev:server", "start"];

/// How the server half is started.
/// Cách nửa server được khởi động.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::commands::core::web::implementation) enum BackendScript {
    /// A Node starter script read through `build_dev_launch`.
    /// Script starter Node đọc qua `build_dev_launch`.
    ///
    /// The name is carried here so the whole launch path keeps its flag policy,
    /// optimizer env, audit log and lifecycle hooks.
    /// Tên được mang theo để toàn đường launch giữ policy flag, env optimizer,
    /// audit log và hook lifecycle.
    NodeScript(&'static str),
    /// A runner started directly, reading its configuration from the env.
    /// Runner chạy trực tiếp, đọc cấu hình từ môi trường.
    ///
    /// These are the backends MagiCore's offline composer writes; none of them
    /// needs a package manager, so no install can be triggered implicitly.
    /// Đây là các backend composer offline ghi ra; không cái nào cần package
    /// manager nên không kích hoạt cài ngầm.
    Runner {
        program: &'static str,
        args: &'static [&'static str],
        label: &'static str,
    },
}

/// Whether `dir` holds a backend half `mgc dev` can start.
/// `dir` có nửa backend mà `mgc dev` khởi động được hay không.
pub(in crate::commands::core::web::implementation) fn is_backend_half(dir: &Path) -> bool {
    BACKEND_MARKERS
        .iter()
        .any(|marker| dir.join(marker).is_file())
}

/// Resolve how the server half at `dir` is started.
/// Resolve cách khởi động nửa server tại `dir`.
///
/// `None` means the directory names no backend MagiCore can start, so the
/// caller reports it instead of spawning an empty process.
/// `None` nghĩa là thư mục không nêu backend nào MagiCore khởi động được, để
/// caller báo thay vì spawn một process rỗng.
pub(in crate::commands::core::web::implementation) fn resolve_backend_script(
    dir: &Path,
) -> Option<BackendScript> {
    if dir.join("package.json").is_file() {
        return node_start_script(dir).map(BackendScript::NodeScript);
    }
    ordered_runners()
        .into_iter()
        .find(|(marker, _)| dir.join(marker).is_file())
        .map(|(_, script)| script)
}

/// Non-Node runners in detection order, each paired with its marker.
/// Runner không Node theo thứ tự nhận diện, mỗi cái kèm marker.
///
/// Order matters: a Rust backend may also ship `src/main.py` fixtures, and the
/// manifest that decides the compile/run tool must win.
/// Thứ tự quan trọng: backend Rust có thể có thêm `src/main.py` cho fixture, nên
/// manifest quyết định công cụ build/run phải thắng.
fn ordered_runners() -> [(&'static str, BackendScript); 7] {
    [
        (
            "go.mod",
            BackendScript::Runner {
                program: "go",
                args: &["run", "."],
                label: "gin server",
            },
        ),
        // A Rust half runs through Cargo's locked/offline lane, so the project
        // must ship the `Cargo.lock` it resolved; MagiCore never unlocks from
        // the dev lane (the executor refuses that), it reports it instead.
        // (Nửa Rust chạy qua lane locked/offline của Cargo nên project phải có
        // `Cargo.lock` đã resolve; lane dev không mở khoá — executor từ chối —
        // mà báo lỗi.)
        (
            "Cargo.toml",
            BackendScript::Runner {
                program: "cargo",
                args: &["run", "--quiet", "--locked"],
                label: "axum server",
            },
        ),
        (
            "manage.py",
            BackendScript::Runner {
                program: PYTHON_LAUNCHER,
                args: &["manage.py", "runserver"],
                label: "django server",
            },
        ),
        (
            "main.py",
            BackendScript::Runner {
                program: PYTHON_LAUNCHER,
                args: &["main.py"],
                label: "python server",
            },
        ),
        (
            "app.py",
            BackendScript::Runner {
                program: PYTHON_LAUNCHER,
                args: &["app.py"],
                label: "flask server",
            },
        ),
        (
            "pyproject.toml",
            BackendScript::Runner {
                program: PYTHON_LAUNCHER,
                args: &["main.py"],
                label: "python server",
            },
        ),
        (
            "src/main.py",
            BackendScript::Runner {
                program: PYTHON_LAUNCHER,
                args: &["src/main.py"],
                label: "python server",
            },
        ),
    ]
}

/// Interpreter the Python halves are started with.
/// Interpreter dùng để khởi động các nửa Python.
#[cfg(windows)]
const PYTHON_LAUNCHER: &str = "python";
/// Interpreter the Python halves are started with.
/// Interpreter dùng để khởi động các nửa Python.
#[cfg(not(windows))]
const PYTHON_LAUNCHER: &str = "python3";

/// First script name a Node starter declares, if any.
/// Tên script đầu tiên mà starter Node khai, nếu có.
fn node_start_script(dir: &Path) -> Option<&'static str> {
    let contents = std::fs::read_to_string(dir.join("package.json")).ok()?;
    let manifest: serde_json::Value = serde_json::from_str(&contents).ok()?;
    let scripts = manifest.get("scripts")?.as_object()?;
    BACKEND_SCRIPTS
        .iter()
        .copied()
        .find(|name| scripts.contains_key(*name))
}

#[cfg(test)]
#[path = "test/backend_script.rs"]
mod tests;
