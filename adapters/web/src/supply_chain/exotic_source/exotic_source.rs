//! Refuse dependency specs that point outside the registry.
//! Từ chối spec dependency trỏ ra ngoài registry.
//!
//! npm and pnpm both publish a `blockExoticSubdeps` policy because a package
//! that depends on `file:`, `git:` or a bare URL can read the machine it is
//! installed on or fetch code the registry never reviewed. Failing with the
//! resolver's "no version matches" message is not the same protection: it looks
//! like a bad version, and it depends on the resolver happening to reject the
//! spec rather than on a rule.
//! npm và pnpm đều có chính sách `blockExoticSubdeps` vì package phụ thuộc
//! `file:`, `git:` hoặc URL trần có thể đọc máy đang cài hoặc tải code registry
//! chưa từng duyệt. Lỗi "không version nào khớp" của resolver không phải cùng
//! một sự bảo vệ: nó trông như version sai và phụ thuộc việc resolver tình cờ
//! từ chối, không dựa trên một luật.
//!
//! This check runs on direct dependencies before resolution, so the failure is
//! named and the policy is a rule rather than a side effect.
//! Kiểm này chạy trên dependency trực tiếp trước khi resolve, nên lỗi được gọi
//! đúng tên và chính sách là một luật, không phải tác dụng phụ.

use mgc_types::MgResult;

/// Spec prefixes that name a source other than the registry.
/// Tiền tố spec nêu nguồn khác registry.
const EXOTIC_PREFIXES: &[&str] = &[
    "file:",
    "link:",
    "git:",
    "git+ssh:",
    "git+https:",
    "git+http:",
    "git+file:",
    "ssh:",
    "http:",
    "https:",
    "github:",
    "gitlab:",
    "bitbucket:",
    "gist:",
    "portal:",
    "workspace:",
];

/// Whether a spec resolves outside the registry.
/// Spec có resolve ra ngoài registry không.
pub fn is_exotic(spec: &str) -> bool {
    let lowered = spec.trim().to_ascii_lowercase();
    if lowered.is_empty() {
        return false;
    }
    // `npm:` aliases stay on the registry; they only rename the package.
    // Alias `npm:` vẫn ở registry; chúng chỉ đổi tên package.
    if lowered.starts_with("npm:") {
        return false;
    }
    if EXOTIC_PREFIXES
        .iter()
        .any(|prefix| lowered.starts_with(prefix))
    {
        return true;
    }
    // A bare `user/repo` shorthand is Git-hosted.
    // Dạng rút gọn `user/repo` là Git.
    is_bare_git_shorthand(&lowered)
}

/// `user/repo`, optionally with `#ref`, without any other marker.
/// `user/repo`, tuỳ chọn `#ref`, không có dấu hiệu nào khác.
fn is_bare_git_shorthand(spec: &str) -> bool {
    let without_ref = spec.split('#').next().unwrap_or(spec);
    let mut parts = without_ref.split('/');
    let (Some(owner), Some(repo), None) = (parts.next(), parts.next(), parts.next()) else {
        return false;
    };
    !owner.is_empty()
        && !repo.is_empty()
        && !owner.contains('@')
        && owner
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        && repo
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
}

/// Reject the first exotic dependency with a named policy error.
///
/// Returns `Ok(())` when every spec stays on the registry.
/// Từ chối dependency exotic đầu tiên bằng lỗi chính sách có tên. Trả `Ok(())`
/// khi mọi spec đều ở registry.
pub fn ensure_registry_only(dependencies: impl Iterator<Item = (String, String)>) -> MgResult<()> {
    let mut refused: Vec<String> = Vec::new();
    for (name, spec) in dependencies {
        if is_exotic(&spec) {
            refused.push(format!("{name}@{spec}"));
        }
    }
    if refused.is_empty() {
        return Ok(());
    }
    refused.sort();
    refused.dedup();
    Err(mgc_types::MgError::Other(format!(
        "exotic dependency source refused: {}. \
         MagiCore resolves dependencies from the registry only, because a spec that points at \
         a path, a Git remote or a URL can read this machine or fetch code no registry reviewed. \
         Depend on a published version, or vendor the code into the project yourself.",
        refused.join(", ")
    )))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
