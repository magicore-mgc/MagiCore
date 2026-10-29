//! Run a non-Web core initializer and verify it produced the expected marker.
//! Chạy initializer core không phải Web và kiểm marker mong đợi.
//!
//! A program that exits zero without writing the framework's marker has not
//! scaffolded anything. Checking the marker is what separates "the command ran"
//! from "the project exists".
//! Chương trình thoát 0 mà không ghi marker của framework thì chưa scaffold gì.
//! Kiểm marker là điều phân biệt "lệnh đã chạy" với "project đã tồn tại".

use std::path::Path;

use anyhow::Result;

use super::super::types::{CoreInitializer, DestinationArg};

/// Run `record` into `destination`, then require `record.marker` to exist.
/// Chạy `record` vào `destination`, rồi bắt buộc `record.marker` phải tồn tại.
pub(crate) fn run(record: &CoreInitializer, destination: &Path) -> Result<()> {
    let probe_args: Vec<String> = record.probe.iter().map(|a| (*a).to_string()).collect();
    let probed = mgc_exec::prelude::run(record.program, &probe_args, &host_options(None))?;
    if probed.exit_code != 0 {
        anyhow::bail!(
            "'{} {}' failed with {}; the toolchain is present but not usable. \
             Install or repair it with: {}",
            record.program,
            record.probe.join(" "),
            probed.exit_code,
            record.install_hint
        );
    }

    // A name-taking scaffolder refuses to run when its destination already
    // exists, so only the parent is prepared here; path-taking tools create the
    // leaf themselves.
    // (Scaffolder nhận tên từ chối chạy khi destination đã tồn tại, nên chỉ
    // chuẩn bị thư mục cha; tool nhận path tự tạo thư mục lá.)
    if let Some(parent) = destination.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }
    let mut args: Vec<String> = record
        .args_before
        .iter()
        .map(|a| (*a).to_string())
        .collect();
    // Where the destination lands depends on what the tool accepts: a
    // positional path, a named flag, or a project name read from the working
    // directory. Sending a path to a tool that wants a name silently succeeds
    // for a directory named after the path components — see `DestinationArg`.
    // (Vị trí destination phụ thuộc tool nhận gì: path vị trí, flag có tên, hay
    // tên project đọc từ thư mục hiện tại. Đưa path cho tool cần tên sẽ "thành
    // công" một cách khó hiểu — xem `DestinationArg`.)
    let cwd: Option<std::path::PathBuf> = match record.destination {
        DestinationArg::Positional => {
            args.push(destination.to_string_lossy().into_owned());
            Some(destination.to_path_buf())
        }
        DestinationArg::Flag(flag) => {
            args.push(flag.to_string());
            args.push(destination.to_string_lossy().into_owned());
            Some(destination.to_path_buf())
        }
        DestinationArg::NameInCwd => {
            // A scaffolder that takes a NAME creates `<cwd>/<name>`, so it must
            // run from the destination's parent; running inside makes it create
            // a nested `<destination>/<name>`.
            // (Scaffolder nhận TÊN tạo `<cwd>/<name>`, nên phải chạy từ thư mục
            // cha của destination; chạy bên trong sẽ tạo `<destination>/<name>`.)
            args.push(project_name(destination)?);
            destination
                .parent()
                .map(Path::to_path_buf)
                .or_else(|| Some(destination.to_path_buf()))
        }
    };
    args.extend(record.args_after.iter().map(|a| (*a).to_string()));
    if let Some(board) = record.board {
        // The project's own choice wins; the catalog value is only a default.
        // Lựa chọn của project thắng; giá trị trong catalog chỉ là mặc định.
        args.push(board.flag.to_string());
        args.push(project_board(destination).unwrap_or_else(|| board.default.to_string()));
    }
    let output = mgc_exec::prelude::run(record.program, &args, &host_options(cwd.as_deref()))?;
    if output.exit_code != 0 {
        let detail = output
            .stderr_tail
            .lines()
            .rev()
            .take(3)
            .collect::<Vec<_>>()
            .join("; ");
        anyhow::bail!(
            "'{}' failed with {}: {detail}",
            record.program,
            output.exit_code
        );
    }

    if !destination.join(record.marker).exists() {
        anyhow::bail!(
            "'{}' exited successfully but '{}' is missing; {}/{} was not scaffolded. \
             The toolchain version may not match what MagiCore expects",
            record.program,
            record.marker,
            record.ecosystem,
            record.framework
        );
    }
    Ok(())
}

/// Exec options for a framework initializer: framework-initializer scope, no
/// inherited environment, output captured, no timeout override.
/// Tuỳ chọn exec cho initializer framework: scope framework-initializer, không
/// kế thừa env, bắt output, không đổi timeout.
fn host_options(cwd: Option<&Path>) -> mgc_exec::prelude::ExecOptions {
    mgc_exec::prelude::ExecOptions {
        cwd: cwd.map(|dir| dir.to_path_buf()),
        clean_env: true,
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::FrameworkInitializer),
        ..Default::default()
    }
}

/// The final path component, which is the project name every scaffolder accepts
/// when it runs inside the destination directory.
/// (Thành phần cuối của đường dẫn — tên project mọi scaffolder nhận khi chạy
/// trong thư mục đích.)
pub(super) fn project_name(destination: &Path) -> Result<String> {
    destination
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .map(str::to_string)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "cannot derive a project name from '{}'; the initializer needs a named project directory",
                destination.display()
            )
        })
}

/// Board or target the project records in `mgc.toml`, if any.
/// Board hoặc target project ghi trong `mgc.toml`, nếu có.
fn project_board(root: &Path) -> Option<String> {
    let raw = std::fs::read_to_string(root.join("mgc.toml")).ok()?;
    for line in raw.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("board") {
            let value = value.trim_start().strip_prefix('=')?.trim();
            let value = value.trim_matches('"');
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
