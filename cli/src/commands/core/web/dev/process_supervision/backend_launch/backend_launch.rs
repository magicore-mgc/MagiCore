//! Builds the launch description of the `server/` half of a fullstack project.
//! Dựng mô tả launch cho nửa `server/` của project fullstack.
//!
//! The runner choice lives in `backend_script`; this owner only turns it into a
//! `DevLaunch`, copying the environment contract the single-framework lane
//! established (`HOST`/`PORT`/`SERVER_PORT`) so a backend reads one convention
//! regardless of which language it is written in.
//! Việc chọn runner nằm ở `backend_script`; owner này chỉ chuyển thành
//! `DevLaunch`, giữ đúng contract môi trường mà lane một framework đã đặt
//! (`HOST`/`PORT`/`SERVER_PORT`) để backend đọc một quy ước dù viết bằng ngôn
//! ngữ nào.

use std::ffi::OsString;
use std::path::PathBuf;

use crate::commands::core::web::implementation::DevLaunch;
use crate::commands::core::web::implementation::is_backend_half;
use crate::commands::core::web::implementation::{
    BackendScript, build_dev_launch, resolve_backend_script,
};

/// Host exported to a runner that binds through the environment.
/// Host xuất cho runner bind qua môi trường.
///
/// The dev frontend runs on another origin, so a loopback-only bind would
/// refuse the browser's preflight. The value is exported, never compiled in.
/// Frontend dev chạy ở origin khác, nên bind loopback-only sẽ từ chối preflight
/// của trình duyệt. Giá trị được xuất qua env, không nhúng vào code.
const BACKEND_DEV_HOST: &str = "127.0.0.1";

/// Build the launch for the server half at `server_dir`.
/// Dựng launch cho nửa server tại `server_dir`.
///
/// A Node starter keeps `build_dev_launch` so the launcher flag policy,
/// optimizer environment, audit log and lifecycle hooks apply unchanged.
/// Starter Node giữ `build_dev_launch` để policy flag, env optimizer, audit log
/// và hook lifecycle áp dụng không đổi.
pub(in crate::commands::core::web::implementation) fn build_backend_launch(
    server_dir: &std::path::Path,
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
) -> anyhow::Result<DevLaunch> {
    let Some(script) = resolve_backend_script(server_dir) else {
        return Err(no_backend_start(server_dir));
    };
    let port = Some(crate::commands::core::dev_port::PORT_WEB_BE);
    match script {
        BackendScript::NodeScript(name) => {
            let mut launch = build_dev_launch(
                server_dir,
                name,
                Some(BACKEND_DEV_HOST.to_string()),
                port,
                compat,
                compat_runner,
            )?;
            append_endpoint_env(&mut launch, port);
            Ok(launch)
        }
        BackendScript::Runner { program, args, .. } => {
            let mut launch = DevLaunch {
                program: PathBuf::from(program),
                args: args.iter().map(OsString::from).collect(),
                envs: Vec::new(),
            };
            append_endpoint_env(&mut launch, port);
            Ok(launch)
        }
    }
}

/// Add the shared endpoint environment to a launch.
/// Thêm môi trường endpoint dùng chung vào một launch.
///
/// `run_dev_launch_with_guard` already exports `HOST`/`PORT`/`SERVER_PORT` and
/// this repeats them only for the Node path, which reads `launch.envs` first.
/// Duplicating the values keeps one source (`dev_port`) rather than a literal.
/// `run_dev_launch_with_guard` đã xuất `HOST`/`PORT`/`SERVER_PORT`; ở đây chỉ lặp
/// cho đường Node vốn đọc `launch.envs` trước. Lặp lại để giữ một nguồn
/// (`dev_port`) thay vì một literal.
fn append_endpoint_env(launch: &mut DevLaunch, port: Option<u16>) {
    launch
        .envs
        .retain(|(key, _)| key != "HOST" && key != "PORT" && key != "SERVER_PORT");
    launch
        .envs
        .push((OsString::from("HOST"), OsString::from(BACKEND_DEV_HOST)));
    if let Some(port) = port {
        let port = OsString::from(port.to_string());
        launch.envs.push((OsString::from("PORT"), port.clone()));
        launch.envs.push((OsString::from("SERVER_PORT"), port));
    }
}

/// Error for a `server/` directory MagiCore cannot start.
/// Lỗi cho thư mục `server/` mà MagiCore không khởi động được.
pub(in crate::commands::core::web::implementation) fn no_backend_start(
    server_dir: &std::path::Path,
) -> anyhow::Error {
    let hint = if is_backend_half(server_dir) {
        "the 'server' directory holds a backend manifest but declares no start command"
    } else {
        "the 'server' directory holds no backend manifest"
    };
    anyhow::anyhow!(
        "{hint}: {} (expected package.json with a dev/start script, pyproject.toml with \
         main.py, manage.py, Cargo.toml, or go.mod). Add the manifest of the language the server \
         is written in, or start it manually.",
        server_dir.display()
    )
}

#[cfg(test)]
#[path = "test/backend_launch.rs"]
mod tests;
