//! Embed a verification statement in the shipped bundle.
//! Nhúng một phát biểu xác minh vào bundle được phát hành.
//!
//! Every other check in this pipeline runs before the artefact leaves the
//! machine. Once it is on a server, nothing knows what it should look like.
//! A short preamble — the build's own identity and the hash of the output SBOM
//! — gives a later reader something to compare against: a deployment can print
//! it, an operator can match it, and a mismatch is visible instead of implied.
//! Mọi kiểm khác trong pipeline chạy trước khi artefact rời máy. Khi nó đã ở
//! trên server, không ai biết nó phải trông thế nào. Một preamble ngắn — danh
//! tính bản build và hash của SBOM output — cho người đọc sau này thứ để so: bản
//! deploy in ra được, người vận hành đối chiếu được, và lệch là thấy chứ không
//! phải suy đoán.
//!
//! This is deliberately not an obfuscated licence check or a phone-home. It is
//! build identity the artefact already has, made visible.
//! Cố ý đây không phải kiểm licence bị làm rối hay gọi về nhà. Nó là danh tính
//! bản build mà artefact vốn có, được làm cho thấy được.

use std::path::Path;

use anyhow::Result;
use sha2::{Digest, Sha256};

/// Prefix every generated preamble carries, so it can be found and replaced.
/// Tiền tố mọi preamble sinh ra đều mang, để tìm và thay được.
pub(crate) const PREAMBLE_MARKER: &str = "/*! magicore-build-identity";

/// The statement embedded in the bundle.
/// Phát biểu được nhúng trong bundle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BuildIdentity {
    /// Project name.
    /// Tên project.
    pub name: String,
    /// Project version.
    /// Phiên bản project.
    pub version: String,
    /// SHA-256 of the output SBOM, hex encoded.
    /// SHA-256 của SBOM output, dạng hex.
    pub sbom_sha256: String,
}

/// Render the preamble as an inert comment plus a global handle.
///
/// A comment alone would be stripped by every minifier, so the value is also
/// assigned to a global: it survives minification and costs one property.
/// Bình luận thuần sẽ bị mọi minifier loại bỏ, nên giá trị cũng được gán cho
/// một global: nó sống qua minify và chỉ tốn một thuộc tính.
pub(crate) fn preamble(identity: &BuildIdentity) -> String {
    format!(
        "{PREAMBLE_MARKER} name={} version={} sbom-sha256={} */\nglobalThis.__MAGICORE_BUILD__={{\"name\":\"{}\",\"version\":\"{}\",\"sbomSha256\":\"{}\"}};",
        identity.name,
        identity.version,
        identity.sbom_sha256,
        escape(&identity.name),
        escape(&identity.version),
        identity.sbom_sha256,
    )
}

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Hash the SBOM file so the preamble names the exact inventory it belongs to.
/// Hash file SBOM để preamble nêu đúng bản kiểm kê nó thuộc về.
pub(crate) fn sbom_sha256(sbom_path: &Path) -> Result<String> {
    let bytes = std::fs::read(sbom_path)?;
    Ok(Sha256::digest(&bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect())
}

/// Prepend the preamble to every JavaScript file in `output_root`.
///
/// Returns the number of files touched so the build can say whether the guard
/// actually landed anywhere.
/// Chèn preamble vào đầu mọi file JavaScript trong `output_root`. Trả số file
/// đã chạm để build nói được guard có thật sự vào đâu không.
pub(crate) fn inject(output_root: &Path, identity: &BuildIdentity) -> Result<usize> {
    let statement = preamble(identity);
    let mut touched = 0;
    for path in javascript_files(output_root)? {
        let text = std::fs::read_to_string(&path)?;
        // A rebuild must not stack preambles: strip the previous one first.
        // Build lại không được chồng preamble: bỏ bản trước trước đã.
        let body = match text.find("*/") {
            Some(end) if text.starts_with(PREAMBLE_MARKER) => &text[end + 2..],
            _ => text.as_str(),
        };
        std::fs::write(&path, format!("{statement}{body}"))?;
        touched += 1;
    }
    Ok(touched)
}

fn javascript_files(root: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();
    if !root.is_dir() {
        return Ok(files);
    }
    for entry in std::fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(javascript_files(&path)?);
        } else if path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| matches!(ext, "js" | "mjs" | "cjs"))
        {
            files.push(path);
        }
    }
    Ok(files)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
