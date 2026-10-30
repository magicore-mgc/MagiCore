//! Run every post-build step over one output directory.
//! Chạy toàn bộ bước sau build trên một thư mục output.
//!
//! Each core produces a different kind of artefact — a directory of scripts, a
//! binary, a wheel — but the question is identical: what exactly ships, and can
//! anyone check it? Keeping the sequence here means a core cannot ship with only
//! half the assurances because its build function forgot a step.
//! Mỗi core tạo một loại artefact khác nhau — thư mục script, binary, wheel —
//! nhưng câu hỏi giống nhau: chính xác cái gì được phát hành, và ai đó kiểm được
//! không? Giữ trình tự ở đây nghĩa là một core không thể phát hành chỉ với nửa
//! số bảo đảm vì hàm build của nó quên một bước.

use std::path::Path;

use anyhow::Result;

use super::super::{
    BuildIdentity, Signed, inject_build_identity, sbom_sha256, scan_output, sign_artifact,
    write_output_sbom,
};

/// What the caller wants done, and what it is willing to fail on.
/// Caller muốn làm gì, và chấp nhận fail ở đâu.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PostBuildOptions<'a> {
    /// Project name recorded in the artefact metadata.
    /// Tên project ghi trong metadata artefact.
    pub name: &'a str,
    /// Project version recorded in the artefact metadata.
    /// Phiên bản project ghi trong metadata artefact.
    pub version: &'a str,
    /// Whether a blocking scan finding stops the build.
    /// Phát hiện chặn có dừng build không.
    pub fail_on_findings: bool,
    /// Whether to embed the build identity into script files.
    ///
    /// Meaningful only for a JavaScript-shaped output; a compiled binary has no
    /// script to prepend to.
    /// Có nhúng danh tính build vào file script không. Chỉ có nghĩa với output
    /// dạng JavaScript; binary đã biên dịch không có script để chèn.
    pub embed_identity: bool,
}

/// Describe, examine, mark and sign `output_dir`.
///
/// Returns `Err` when the scan found a blocking issue and the caller asked to
/// fail on it.
/// Mô tả, xét, đánh dấu và ký `output_dir`. Trả `Err` khi scan tìm thấy vấn đề
/// chặn và caller yêu cầu fail.
pub(crate) fn run(output_dir: &Path, options: PostBuildOptions<'_>) -> Result<()> {
    if !output_dir.is_dir() {
        // W-1b: reporting success for a directory that does not exist means the
        // SBOM, the scan and the signature all describe nothing while the build
        // says it succeeded. A core that chooses another output directory (the
        // framework build lanes write wherever the framework puts its result)
        // must name its real directory instead of silencing the check.
        // (W-1b: báo thành công cho thư mục không tồn tại nghĩa là SBOM, scan và
        // chữ ký đều mô tả hư không còn build báo thành công. Core dùng thư mục
        // output khác (lane framework build ghi nơi framework đặt kết quả) phải
        // nêu đúng thư mục của nó thay vì tắt bước kiểm.)
        anyhow::bail!(
            "post-build failed: build output '{}' does not exist, so the SBOM, scan and signature would describe nothing",
            output_dir.display()
        );
    }

    // Order matters: the identity guard changes the bytes of every script, so
    // the SBOM must be written last. Hashing before injection produced a
    // document that described files which no longer existed.
    // Thứ tự quan trọng: guard đổi byte của mọi script, nên SBOM phải ghi sau
    // cùng. Hash trước khi inject tạo tài liệu mô tả file không còn tồn tại.
    //
    // The guard needs the SBOM hash, so it is embedded twice: a placeholder
    // first keeps the byte length stable, then the real hash is written.
    // Guard cần hash SBOM, nên nó được nhúng hai lần: placeholder trước giữ độ
    // dài byte ổn định, rồi ghi hash thật.
    if options.embed_identity {
        inject_build_identity(
            output_dir,
            &BuildIdentity {
                name: options.name.to_string(),
                version: options.version.to_string(),
                sbom_sha256: "0".repeat(64),
            },
        )?;
    }

    let report = scan_output(output_dir)?;
    mgc_ui::info(&report.summary());
    if report.has_blocking() && options.fail_on_findings {
        anyhow::bail!(
            "build output failed the security scan; fix the findings above or pass --allow-output-warnings to ship anyway"
        );
    }

    let sbom_path = write_output_sbom(output_dir, options.name, options.version)?;
    let sbom_digest = sbom_sha256(&sbom_path)?;

    if options.embed_identity {
        inject_build_identity(
            output_dir,
            &BuildIdentity {
                name: options.name.to_string(),
                version: options.version.to_string(),
                sbom_sha256: sbom_digest,
            },
        )?;
        // The second pass changed the bytes the SBOM recorded, so it is
        // regenerated once more with the final content.
        // Lượt hai đã đổi byte mà SBOM đã ghi, nên nó được sinh lại lần cuối.
        write_output_sbom(output_dir, options.name, options.version)?;
    }

    mgc_ui::info(&format!("Output SBOM: {}", sbom_path.display()));

    // Signing resolves the `mgc trust` keyring first (one source of truth for
    // `mgc trust init` and `mgc build`); `data_dir` only covers the legacy
    // single-seed file, which the resolver warns about.
    // (Ký ưu tiên keyring `mgc trust` — một nguồn chân lý cho `mgc trust init` và
    // `mgc build`); `data_dir` chỉ cho file một-seed kiểu cũ, resolver có cảnh báo.)
    match sign_artifact(&sbom_path, &mgc_platform::paths::mgc_data_dir())? {
        Signed::Signature(path) => mgc_ui::info(&format!("Signed: {}", path.display())),
        Signed::Skipped(reason) => mgc_ui::info(&format!("Output not signed: {reason}")),
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

/// Run post-build over the artefact directory a core produced.
///
/// The directory differs per core: a compiled binary is a file, so its parent
/// is described; a build tree is a directory. Callers pass what they produced.
/// Chạy sau build trên thư mục artefact mà core tạo. Thư mục khác nhau theo
/// core: binary đã biên dịch là một file nên mô tả thư mục cha; cây build là
/// thư mục. Caller truyền đúng thứ nó tạo ra.
pub(crate) fn run_for_core(
    root: &Path,
    core: &str,
    output: &Path,
    allow_warnings: bool,
) -> Result<()> {
    let name = root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| core.to_string());
    // Record the project's real version, not a fake constant: the SBOM and the
    // signed manifest must describe the artefact actually built. Fall back to
    // "0.0.0" (a visibly-unknown version) when the project has no config, so a
    // missing config never masquerades as a released 1.0.0.
    // Ghi đúng phiên bản thật của project, không dùng hằng giả: SBOM và manifest
    // ký phải mô tả đúng artefact đã build. Fallback "0.0.0" (phiên bản hiển thị
    // là chưa biết) khi project không có config — tránh giả thành 1.0.0 đã phát hành.
    let version = project_version(root);
    run(
        output,
        PostBuildOptions {
            name: &name,
            version: &version,
            fail_on_findings: !allow_warnings,
            // Only a JavaScript-shaped output has scripts to prepend to.
            // Chỉ output dạng JavaScript mới có script để chèn.
            embed_identity: matches!(core, "web"),
        },
    )
}

// Read the project version from `mgc.toml`. Only an explicitly recorded
// `version` key is used; if the file is absent, unreadable, or omits `version`,
// return a visibly-unknown "0.0.0" rather than the schema's serde default
// (which is a hardcoded placeholder, not the project's real version).
// Đọc phiên bản project từ `mgc.toml`. Chỉ dùng key `version` được ghi tường
// minh; nếu thiếu file, không đọc được, hoặc thiếu `version`, trả "0.0.0" hiển
// thị chưa biết — không dùng serde default (vốn là hằng giả, không phải version
// thật của project).
pub(crate) fn project_version(root: &Path) -> String {
    let path = root.join("mgc.toml");
    let Ok(content) = std::fs::read_to_string(&path) else {
        return "0.0.0".to_string();
    };
    let Ok(document) = content.parse::<toml::Value>() else {
        return "0.0.0".to_string();
    };
    match document.get("version").and_then(toml::Value::as_str) {
        Some(version) if !version.trim().is_empty() => version.to_string(),
        _ => "0.0.0".to_string(),
    }
}
