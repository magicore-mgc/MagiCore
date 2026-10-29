//! Examine the built output for the shapes an incident leaves behind.
//! Xét output đã build tìm những dạng mà một sự cố để lại.
//!
//! Integrity checking stops at install: after a package is verified, whatever
//! the build then does is trusted. That gap is where a compromised plugin or a
//! postinstall that rewrote a source file shows up — the dependency was clean
//! when it was fetched, and the artefact is not. These checks look at the
//! bytes that actually ship.
//! Kiểm toàn vẹn dừng ở bước cài: sau khi package đã verify, build làm gì tiếp
//! đều được tin. Khe đó là nơi plugin bị compromise hoặc postinstall đã ghi lại
//! source xuất hiện — dependency sạch lúc tải, artefact thì không. Các kiểm này
//! xét đúng byte sẽ phát hành.

use std::path::Path;

use anyhow::Result;

/// How serious a finding is.
/// Mức nghiêm trọng của một phát hiện.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Severity {
    /// Worth knowing, never blocks a build.
    /// Đáng biết, không chặn build.
    Note,
    /// Blocks a build unless the caller opts out.
    /// Chặn build trừ khi caller bỏ qua.
    Error,
}

/// One thing the scan found.
/// Một điều scan tìm thấy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Finding {
    pub severity: Severity,
    /// File the finding is in, relative to the output root.
    /// File chứa phát hiện, tương đối so với root output.
    pub file: String,
    /// Short rule id, stable enough to grep in CI logs.
    /// Id luật ngắn, đủ ổn định để grep trong log CI.
    pub rule: &'static str,
    /// Human explanation.
    /// Giải thích cho người đọc.
    pub detail: String,
}

/// Result of scanning the whole output.
/// Kết quả scan toàn bộ output.
#[derive(Debug, Default)]
pub(crate) struct Report {
    pub findings: Vec<Finding>,
}

impl Report {
    /// Whether any finding blocks the build.
    /// Có phát hiện nào chặn build không.
    pub(crate) fn has_blocking(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity == Severity::Error)
    }

    /// Group findings by rule for a compact summary.
    /// Gom phát hiện theo luật để tóm tắt gọn.
    pub(crate) fn summary(&self) -> String {
        if self.findings.is_empty() {
            return "output scan: no findings".to_string();
        }
        let mut lines = Vec::with_capacity(self.findings.len() + 1);
        lines.push(format!("output scan: {} finding(s)", self.findings.len()));
        for finding in &self.findings {
            lines.push(format!(
                "  {} [{}] {}: {}",
                match finding.severity {
                    Severity::Error => "error",
                    Severity::Note => "note",
                },
                finding.rule,
                finding.file,
                finding.detail
            ));
        }
        lines.join("\n")
    }
}

/// Text extensions worth reading. Binary assets cannot be judged this way and
/// are covered by the output SBOM's hashes instead.
/// Đuôi văn bản đáng đọc. Asset nhị phân không đánh giá được theo cách này và
/// đã được hash trong SBOM output.
// `map` is scanned because a source map is precisely the artefact that embeds
// absolute build-machine paths; skipping it meant the `absolute-source-path`
// rule could only ever fire on an inline map.
// (`map` được scan vì source map chính là artefact nhúng đường dẫn build tuyệt
// đối; bỏ qua nó khiến rule `absolute-source-path` chỉ bắt được map inline.)
const TEXT_EXTENSIONS: [&str; 7] = ["js", "mjs", "cjs", "css", "html", "json", "map"];

/// Scan every text file under `output_root`.
/// Scan mọi file văn bản dưới `output_root`.
pub(crate) fn scan(output_root: &Path) -> Result<Report> {
    let mut report = Report::default();
    if !output_root.is_dir() {
        return Ok(report);
    }
    for entry in walk(output_root)? {
        let relative = entry
            .strip_prefix(output_root)
            .unwrap_or(&entry)
            .to_string_lossy()
            .replace('\\', "/");
        let extension = entry
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        if !TEXT_EXTENSIONS.contains(&extension.as_str()) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&entry) else {
            continue;
        };
        inspect_text(&relative, &text, &mut report);
    }
    Ok(report)
}

fn walk(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = std::fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_dir() {
            files.extend(walk(&path)?);
        } else if metadata.is_file() {
            files.push(path);
        }
    }
    Ok(files)
}

fn inspect_text(relative: &str, text: &str, report: &mut Report) {
    // `eval` with a non-literal argument is the classic injection sink: the
    // argument can come from anywhere, including a response body.
    // `eval` với đối số không phải literal là sink injection kinh điển: đối số
    // có thể đến từ bất kỳ đâu, kể cả body response.
    for (index, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("eval(") || trimmed.contains("eval(") {
            let after = &trimmed[trimmed.find("eval(").map(|i| i + 5).unwrap_or(0)..];
            let literal = after.starts_with('"')
                || after.starts_with('\'')
                || after.starts_with('`')
                || after.starts_with('[')
                || after.starts_with('{');
            if !literal {
                report.findings.push(Finding {
                    severity: Severity::Error,
                    file: relative.to_string(),
                    rule: "dynamic-eval",
                    detail: format!("eval() on line {} with a non-literal argument", index + 1),
                });
                break;
            }
        }
    }

    // A development build that reached production still carries its runtime
    // assertions; that is a correctness smell, not a vulnerability.
    // Bản dev lọt vào production vẫn mang assertion runtime; đây là mùi đúng
    // đắn, không phải lỗ hổng.
    if text.contains("react-dom.development") || text.contains("checkPropTypes") {
        report.findings.push(Finding {
            severity: Severity::Note,
            file: relative.to_string(),
            rule: "development-build",
            detail: "development build shipped; NODE_ENV was not production at build time"
                .to_string(),
        });
    }

    // A source map that claims `sources` outside the project leaks build paths.
    // Every platform's home prefix counts: checking only `/Users/` (macOS) let a
    // Linux or Windows build machine path ship undetected, so the same leak was
    // reported on one OS and silent on the other two.
    // (Mọi prefix home của từng nền tảng đều được tính: chỉ kiểm `/Users/` khiến
    // đường dẫn máy build Linux/Windows lọt mà không báo.)
    if text.contains("webpack://")
        && ABSOLUTE_BUILD_PATH_MARKERS
            .iter()
            .any(|marker| text.contains(marker))
    {
        report.findings.push(Finding {
            severity: Severity::Note,
            file: relative.to_string(),
            rule: "absolute-source-path",
            detail: "output embeds an absolute build-machine path".to_string(),
        });
    }
}

/// Home-prefix markers of a developer machine, one per supported OS.
///
/// Kept as data rather than inline literals so a new platform is one entry and
/// the scanner cannot silently cover only the author's OS.
/// (Marker prefix home của máy dev, mỗi OS một mục. Để dạng dữ liệu để thêm nền
/// tảng mới chỉ là thêm một dòng và scanner không lặng lẽ chỉ phủ OS của tác giả.)
const ABSOLUTE_BUILD_PATH_MARKERS: &[&str] = &["/Users/", "/home/", "C:\\Users\\", "/root/"];

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
