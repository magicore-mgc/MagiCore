//! Run a scaffold program the user already has on `PATH`.
//! Chạy chương trình scaffold mà user đã có trên `PATH`.
//!
//! Django, Laravel and Rails publish no npm package and no generator service in
//! the shape MagiCore can call over HTTP. Their scaffolding program is part of
//! the language toolchain the developer already installed
//! (`django-admin`, `composer`, `rails`). MagiCore runs that program directly
//! and never supplies files of its own.
//! Django, Laravel và Rails không phát hành package npm, cũng không có dịch vụ
//! sinh project theo dạng MagiCore gọi được qua HTTP. Chương trình scaffold của
//! chúng thuộc toolchain ngôn ngữ mà lập trình viên đã cài sẵn. MagiCore chạy
//! chương trình đó trực tiếp và không tự cung cấp file nào.
//!
//! The probe comes first. A missing or broken toolchain produces a clear message
//! with the exact install command, instead of a half-written project.
//! Probe chạy trước. Toolchain thiếu hoặc hỏng cho thông điệp rõ ràng kèm đúng
//! lệnh cài, thay vì để lại project viết dở.

use std::path::Path;

use anyhow::{Context, Result};

/// Run the version probe, then the initializer, into `destination`.
///
/// Returns an error naming the install hint when the program is absent or the
/// probe fails, so adoption of an unproven toolchain is impossible.
/// Chạy probe phiên bản, rồi chạy initializer vào `destination`. Trả lỗi kèm
/// gợi ý cài khi thiếu chương trình hoặc probe hỏng, nên không thể dùng một
/// toolchain chưa được chứng minh.
pub(in crate::commands::core::web::implementation) fn run_host_tool(
    program: &str,
    install_hint: &str,
    probe: &[&str],
    args: &[String],
    destination: &Path,
) -> Result<()> {
    // Probe: the program must exist and answer. `--version` alone is enough to
    // prove the toolchain is usable; MagiCore does not parse the output.
    // Probe: chương trình phải tồn tại và trả lời. Chỉ `--version` là đủ để
    // chứng minh toolchain dùng được; MagiCore không parse output.
    let probe_args: Vec<String> = probe.iter().map(|a| (*a).to_string()).collect();
    let probed =
        mgc_exec::prelude::run(program, &probe_args, &host_options(None)).map_err(|error| {
            anyhow::anyhow!(
                "'{program}' is required to scaffold this framework but could not be run: {error}. \
             Install it with: {install_hint}"
            )
        })?;
    if probed.exit_code != 0 {
        anyhow::bail!(
            "'{program} {}' failed with {}; the toolchain is present but not usable. \
             Install or repair it with: {install_hint}",
            probe.join(" "),
            probed.exit_code
        );
    }

    std::fs::create_dir_all(destination)?;
    let output = mgc_exec::prelude::run(program, args, &host_options(Some(destination)))
        .with_context(|| format!("run '{program} {}'", args.join(" ")))?;
    // The probe already proved the toolchain works, so a failure past this
    // point is the initializer rejecting its arguments, not a missing tool.
    // Probe đã chứng minh toolchain chạy được, nên lỗi sau đây là initializer
    // từ chối tham số, không phải thiếu tool.
    if output.exit_code != 0 {
        let detail = output
            .stderr_tail
            .lines()
            .rev()
            .take(3)
            .collect::<Vec<_>>()
            .join("; ");
        anyhow::bail!(
            "'{program} {}' failed with {}: {detail}",
            args.join(" "),
            output.exit_code
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

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
