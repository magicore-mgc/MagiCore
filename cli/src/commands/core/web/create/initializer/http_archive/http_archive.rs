//! Download a project archive from the framework's own generator service.
//! Tải archive project từ dịch vụ sinh project của chính framework.
//!
//! Some frameworks publish no npm initializer but do run a public generator
//! service that renders a project on request: `start.spring.io` for Spring
//! Boot, `code.quarkus.io` for Quarkus. Calling that service is the framework
//! team's own scaffolding path — MagiCore adds no template and needs no local
//! toolchain, only an HTTP round trip.
//! Vài framework không phát hành initializer npm nhưng có dịch vụ sinh project
//! công khai render project theo yêu cầu: `start.spring.io` cho Spring Boot,
//! `code.quarkus.io` cho Quarkus. Gọi dịch vụ đó chính là đường scaffold của
//! framework team — MagiCore không thêm template, không cần toolchain local,
//! chỉ một vòng HTTP.

use std::io::Cursor;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result};

/// Run one HTTP-archive initializer into `destination`.
///
/// The archive is extracted into `destination` with every entry validated
/// first, so a malicious or buggy service cannot write outside the scaffold
/// workspace.
/// Chạy một initializer dạng archive HTTP vào `destination`. Archive được giải
/// nén vào `destination` sau khi kiểm từng entry, nên dịch vụ lỗi hay độc hại
/// không ghi được ra ngoài workspace scaffold.
pub(in crate::commands::core::web::implementation) async fn run_http_archive(
    endpoint: &str,
    params: &[(&str, &str)],
    name_param: &str,
    project_name: &str,
    destination: &Path,
) -> Result<()> {
    let url = build_url(endpoint, params, name_param, project_name);

    let response = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .context("build generator-service client")?
        .get(&url)
        .header("accept", "application/zip")
        .send()
        .await
        .with_context(|| format!("request project archive from {endpoint}"))?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!(
            "generator service {endpoint} returned {status} for '{project_name}'; \
             MagiCore asks the framework's own service and does not fall back to a template"
        );
    }

    let bytes = response
        .bytes()
        .await
        .with_context(|| format!("read project archive from {endpoint}"))?;

    let mut archive = zip::ZipArchive::new(Cursor::new(bytes))
        .with_context(|| format!("{endpoint} did not return a zip archive"))?;

    // Validate every entry before writing anything: a partial extract of a
    // hostile archive is worse than no project at all.
    // Kiểm mọi entry trước khi ghi: giải nén một phần archive độc hại còn tệ
    // hơn không có project nào.
    for index in 0..archive.len() {
        let entry = archive.by_index(index)?;
        let name = entry.name()?.to_string();
        if safe_relative_path(&name).is_none() {
            anyhow::bail!("generator service {endpoint} produced an unsafe archive entry '{name}'");
        }
    }

    std::fs::create_dir_all(destination)?;
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index)?;
        let entry_name = entry.name()?.to_string();
        let Some(relative) = safe_relative_path(&entry_name) else {
            continue;
        };
        let out_path: PathBuf = destination.join(relative);
        // Re-check the resolved path: a component that survives the first pass
        // still must not escape after joining.
        // Kiểm lại đường dẫn sau khi join: thành phần qua được vòng đầu vẫn
        // không được thoát ra ngoài.
        if !out_path.starts_with(destination) {
            continue;
        }
        if entry.is_dir() {
            std::fs::create_dir_all(&out_path)?;
            continue;
        }
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = std::fs::File::create(&out_path)?;
        std::io::copy(&mut entry, &mut file)?;
    }
    Ok(())
}

/// Build the request URL with the project name in its declared slot.
/// Dựng URL yêu cầu với tên project nằm đúng tham số đã khai báo.
fn build_url(
    endpoint: &str,
    params: &[(&str, &str)],
    name_param: &str,
    project_name: &str,
) -> String {
    let encoded_name = urlencode(project_name);
    let mut pairs: Vec<String> = params
        .iter()
        .map(|(key, value)| format!("{}={}", urlencode(key), urlencode(value)))
        .collect();
    pairs.push(format!("{}={encoded_name}", urlencode(name_param)));
    format!("{endpoint}?{}", pairs.join("&"))
}

/// Minimal percent-encoding for query values.
/// Percent-encoding tối thiểu cho giá trị query.
fn urlencode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

/// Reject absolute paths, parent traversal and drive prefixes from an archive
/// entry name.
/// Từ chối đường dẫn tuyệt đối, đi lên thư mục cha và tiền tố ổ đĩa từ tên
/// entry trong archive.
fn safe_relative_path(name: &str) -> Option<PathBuf> {
    // A Windows drive prefix is absolute on Windows but looks relative on Unix,
    // so it must be rejected by shape, not by `Component`.
    // Tiền tố ổ đĩa Windows là tuyệt đối trên Windows nhưng trông tương đối
    // trên Unix, nên phải chặn theo hình dạng chứ không dựa vào `Component`.
    let bytes = name.as_bytes();
    if bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
        return None;
    }
    let normalised = name.replace('\\', "/");
    let candidate = Path::new(&normalised);
    let mut out = PathBuf::new();
    for component in candidate.components() {
        match component {
            Component::Normal(part) => {
                let part = part.to_string_lossy();
                if part.is_empty() || part == "." || part == ".." {
                    return None;
                }
                out.push(part.as_ref());
            }
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    if out.as_os_str().is_empty() {
        None
    } else {
        Some(out)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
