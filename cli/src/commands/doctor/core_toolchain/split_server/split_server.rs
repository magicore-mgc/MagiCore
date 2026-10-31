//! Adds the toolchain a split server half needs to the `web` core row.
//! Thêm toolchain mà nửa server tách rời cần vào hàng core `web`.
//!
//! A `web` project's frontend half runs on Node, but its `server/` half is
//! started by its own runner — `python3` for FastAPI/Flask/Django, `cargo` for
//! Axum, `go` for Gin. Reporting HEALTHY without probing that runner would claim
//! readiness for a half the machine cannot start.
//! Nửa frontend của project `web` chạy trên Node, nhưng nửa `server/` do runner
//! riêng khởi động — `python3` cho FastAPI/Flask/Django, `cargo` cho Axum, `go`
//! cho Gin. Báo HEALTHY mà không probe runner đó là tuyên bố sẵn sàng cho nửa mà
//! máy không khởi động được.

use std::path::Path;

/// Toolchains a split server half can require, in display order.
/// Toolchain mà nửa server tách rời có thể yêu cầu, theo thứ tự hiển thị.
///
/// Each entry is (manifest that names the language, entry the runner executes,
/// required binaries). The entry must exist too: a manifest alone is not a
/// server MagiCore can start.
/// Mỗi entry là (manifest nêu ngôn ngữ, entry mà runner chạy, binary bắt buộc).
/// Entry cũng phải tồn tại: manifest trần không phải server MagiCore khởi động được.
const SERVER_HALF_TOOLCHAINS: &[(&str, &str, &[&str])] = &[
    ("pyproject.toml", "main.py", &["python3"]),
    ("pyproject.toml", "app.py", &["python3"]),
    ("pyproject.toml", "manage.py", &["python3"]),
    ("requirements.txt", "main.py", &["python3"]),
    ("Cargo.toml", "src/main.rs", &["cargo"]),
    ("go.mod", "main.go", &["go"]),
];

/// Binaries the split server half of `project_root` needs.
/// Binary mà nửa server tách rời của `project_root` cần.
///
/// Returns an empty slice when the project has no startable server half, or when
/// its server root is the project root itself (a bare backend project, which the
/// core row already covers).
/// Trả slice rỗng khi project không có nửa server khởi động được, hoặc khi gốc
/// server chính là gốc project (project backend trần — hàng core đã bao phủ).
pub(in crate::commands::doctor) fn split_server_half_tools(
    project_root: &Path,
) -> &'static [&'static str] {
    let server_root = project_root.join("server");
    if !server_root.is_dir() {
        return &[];
    }
    SERVER_HALF_TOOLCHAINS
        .iter()
        .find(|(manifest, entry, _)| {
            server_root.join(manifest).is_file() && server_root.join(entry).is_file()
        })
        .map(|(_, _, tools)| *tools)
        .unwrap_or(&[])
}
