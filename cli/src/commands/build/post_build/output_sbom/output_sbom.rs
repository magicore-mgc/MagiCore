//! Describe the built output as a CycloneDX component tree.
//! Mô tả output đã build bằng cây component CycloneDX.
//!
//! MagiCore already writes an SBOM of the dependencies it installed. That
//! answers "what went in"; this answers "what came out" — the artefact that
//! actually ships, file by file, hash by hash. Without it a deployment can be
//! compared against nothing: a server cannot ask "is this the bundle we built",
//! and an incident report cannot say which files changed.
//! MagiCore đã ghi SBOM của dependency đã cài. Cái đó trả lời "cái gì đi vào";
//! cái này trả lời "cái gì đi ra" — artefact thật sự được phát hành, từng file,
//! từng hash. Thiếu nó thì không so được bản deploy với gì: server không hỏi
//! được "đây có đúng bundler ta đã build", và báo cáo sự cố không nói được file
//! nào đã đổi.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;
use sha2::{Digest, Sha256};

/// One file in the built output.
/// Một file trong output đã build.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OutputFile {
    /// Path relative to the output directory, with forward slashes.
    /// Đường dẫn tương đối so với thư mục output, dùng dấu gạch chéo.
    pub path: String,
    /// SHA-256 of the file contents, hex encoded.
    /// SHA-256 nội dung file, dạng hex.
    pub sha256: String,
    /// Size in bytes.
    /// Kích thước tính bằng byte.
    pub bytes: u64,
}

/// Hash every file under `output_root`.
///
/// Symlinks are followed only when they resolve inside the output directory:
/// an artefact must not describe a file it does not ship.
/// Hash mọi file dưới `output_root`. Symlink chỉ được đi theo khi resolve bên
/// trong thư mục output: artefact không được mô tả file nó không phát hành.
pub(crate) fn hash_output(output_root: &Path) -> Result<Vec<OutputFile>> {
    let root = output_root.canonicalize()?;
    let mut files = Vec::new();
    collect(&root, &root, &mut files)?;
    files.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(files)
}

fn collect(root: &Path, dir: &Path, out: &mut Vec<OutputFile>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = std::fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() {
            let Ok(resolved) = path.canonicalize() else {
                continue;
            };
            if !resolved.starts_with(root) {
                continue;
            }
            if resolved.is_file() {
                push_file(root, &path, &resolved, out)?;
            }
            continue;
        }
        if metadata.is_dir() {
            collect(root, &path, out)?;
        } else if metadata.is_file() {
            push_file(root, &path, &path, out)?;
        }
    }
    Ok(())
}

fn push_file(root: &Path, logical: &Path, real: &Path, out: &mut Vec<OutputFile>) -> Result<()> {
    let bytes = std::fs::read(real)?;
    let relative = logical
        .strip_prefix(root)
        .unwrap_or(logical)
        .to_string_lossy()
        .replace('\\', "/");
    out.push(OutputFile {
        path: relative,
        sha256: hex(&Sha256::digest(&bytes)),
        bytes: bytes.len() as u64,
    });
    Ok(())
}

/// Render the files as a CycloneDX 1.5 JSON document.
/// Render các file thành tài liệu CycloneDX 1.5 JSON.
pub(crate) fn to_cyclonedx(name: &str, version: &str, files: &[OutputFile]) -> serde_json::Value {
    let components: Vec<serde_json::Value> = files
        .iter()
        .map(|file| {
            let mut hashes = BTreeMap::new();
            hashes.insert("SHA-256", file.sha256.clone());
            serde_json::json!({
                "type": "file",
                "name": file.path,
                "hashes": hashes
                    .into_iter()
                    .map(|(alg, content)| serde_json::json!({"alg": alg, "content": content}))
                    .collect::<Vec<_>>(),
                "properties": [{"name": "magicore:bytes", "value": file.bytes.to_string()}],
            })
        })
        .collect();

    serde_json::json!({
        "bomFormat": "CycloneDX",
        "specVersion": "1.5",
        "version": 1,
        "metadata": {
            "component": {
                "type": "application",
                "name": name,
                "version": version,
            },
            "tools": [{"vendor": "MagiCore", "name": "mgc"}],
        },
        "components": components,
    })
}

/// Write the output SBOM next to the built files.
/// Ghi SBOM output cạnh các file đã build.
pub(crate) fn write(output_root: &Path, name: &str, version: &str) -> Result<std::path::PathBuf> {
    let files = hash_output(output_root)?;
    let document = to_cyclonedx(name, version, &files);
    let path = output_root.join("sbom.output.cdx.json");
    std::fs::write(&path, serde_json::to_vec_pretty(&document)?)?;
    Ok(path)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

/// Lowercase hex encoding of a digest.
/// Mã hoá hex chữ thường của một digest.
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
