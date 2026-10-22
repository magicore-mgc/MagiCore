use mgc_types::MgError;

/// Detect Git-source names only so resolution can reject them without
/// spawning an external Git process.
/// Dò tên git: tên registry là `scope/name`; mọi tên có segment đầu kiểu
/// host hoặc đuôi `.git` là dep chỉ-git. `file://` KHÔNG được nhận diện.
pub fn is_git_name(name: &str) -> bool {
    // A scheme other than https (file/http/ssh/git) is never a git
    // transport candidate — rejected here so it can fail explicitly
    // downstream instead of riding the git path.
    // (Scheme khác https không bao giờ là ứng viên transport git.)
    if name.contains("://") && !name.starts_with("https://") {
        return false;
    }
    name.ends_with(".git")
        || name.matches('/').count() > 1
        || name
            .split('/')
            .next()
            .is_some_and(|first| first.contains('.') && !first.contains(".."))
}

/// Map a `.package(url:)` value to the graph-edge name (https scheme +
/// `.git` stripped; anything else rides through unchanged and fails
/// explicitly at resolve time — no silent acceptance).
/// Ánh xạ giá trị `.package(url:)` sang tên cạnh graph.
pub fn git_url_to_name(url: &str) -> String {
    let url = url.trim();
    if let Some(rest) = url.strip_prefix("https://") {
        rest.trim_end_matches('/')
            .strip_suffix(".git")
            .unwrap_or(rest.trim_end_matches('/'))
            .to_string()
    } else {
        url.to_string()
    }
}

/// Git transport is disabled until a native HTTP implementation exists;
/// no allowlist or environment variable enables an external Git process.
/// Transport Git bị tắt cho tới khi có native HTTP implementation.
pub fn git_transport_allowed(
    _url: &str,
    _allow_git_deps: bool,
    _hosts: &[&str],
) -> Result<(), MgError> {
    Err(MgError::Unsupported {
        core: "app",
        capability: "native Swift Git transport",
        guidance: "Git transport is disabled; MGC_GIT_DEPS and host allowlists do not enable an external Git executable".to_string(),
    })
}

/// Repository display name (last path segment, `.git` stripped).
/// Tên hiển thị repo (segment cuối, bỏ `.git`).
#[cfg(test)]
pub(in crate::protocols::swift) fn git_repo_name(name: &str) -> String {
    name.trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(name)
        .trim_end_matches(".git")
        .trim_end_matches('/')
        .to_string()
}

/// Registry identity → graph name: `scope.name` → `scope/name`; a name
/// that already uses `/` rides as-is; a scope-less identity stays as-is and
/// fails closed at resolve time with guidance.
/// Registry identity → tên graph: `scope.name` → `scope/name`; tên đã dùng
/// `/` giữ nguyên; identity không scope giữ nguyên và fail-closed lúc
/// resolve kèm hướng dẫn.
pub(in crate::protocols::swift) fn identity_to_dep_name(identity: &str) -> String {
    let identity = identity.trim();
    if identity.contains('/') || !identity.contains('.') {
        return identity.to_string();
    }
    match identity.split_once('.') {
        Some((scope, name)) if !scope.is_empty() && !name.is_empty() => {
            format!("{scope}/{name}")
        }
        _ => identity.to_string(),
    }
}
