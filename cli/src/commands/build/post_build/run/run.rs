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

    let keyring = mgc_platform::paths::mgc_data_dir().join("keyring.json");
    match sign_artifact(&sbom_path, &keyring)? {
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
    run(
        output,
        PostBuildOptions {
            name: &name,
            version: "1.0.0",
            fail_on_findings: !allow_warnings,
            // Only a JavaScript-shaped output has scripts to prepend to.
            // Chỉ output dạng JavaScript mới có script để chèn.
            embed_identity: matches!(core, "web"),
        },
    )
}
