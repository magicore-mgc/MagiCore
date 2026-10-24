//! Manifest and lock update after audit-fix resolution succeeds.
//! Cập nhật manifest và lock sau khi resolve audit-fix thành công.

use mgc_types::{DependencySpec, MgError, MgResult, PackageId};
use std::path::Path;

use crate::lockfile::write_web_lockfile_with_state;
use crate::manifest::parse_manifest;

pub async fn run_audit_fix<F, Fut>(
    project_root: &Path,
    vulnerable: &[PackageId],
    resolve_fn: F,
) -> MgResult<usize>
where
    F: FnOnce(mgc_types::Manifest) -> Fut,
    Fut: std::future::Future<Output = MgResult<mgc_types::adapter::ResolvedGraph>>,
{
    if vulnerable.is_empty() {
        return Ok(0);
    }

    let manifest = parse_manifest(project_root)?;
    let names: std::collections::HashSet<&str> =
        vulnerable.iter().map(|id| id.name_str()).collect();

    let flags = [
        (false, false, false),
        (true, false, false),
        (false, false, true),
        (false, true, false),
    ];
    let mut bumps: Vec<DependencySpec> = Vec::new();
    for (group, (dev, optional, peer)) in manifest.dep_groups().iter().zip(flags) {
        for spec in group.1 {
            let _ = (dev, optional, peer);
            if names.contains(spec.name.as_str()) {
                bumps.push(spec.clone());
            }
        }
    }
    if bumps.is_empty() {
        return Err(MgError::Other(
            "audit --fix: found no matching dependency to bump (validate manifest)".into(),
        ));
    }

    // A fix must never widen the requirement. Replacing a pinned range with
    // `*` "clears" the advisory by deleting every constraint the author chose,
    // which is a worse supply-chain posture than the vulnerable pin itself.
    // The ranges are kept exactly as written; the re-resolve below decides.
    // Bản vá không bao giờ được nới rộng yêu cầu. Thay range đã pin bằng `*`
    // là "xoá" advisory bằng cách xoá mọi ràng buộc tác giả đã chọn — tệ hơn
    // chính bản pin dính lỗ hổng. Range được giữ nguyên; resolve lại bên dưới
    // sẽ quyết định.
    let pinned_to_vulnerable: Vec<String> = bumps
        .iter()
        .filter(|spec| spec.range.is_star())
        .map(|spec| format!("{}@*", spec.name))
        .collect();
    if !pinned_to_vulnerable.is_empty() {
        return Err(MgError::Other(format!(
            "audit --fix: the manifest already uses an unbounded version range for {}. \
             MagiCore needs a constraint that can exclude the vulnerable release; add one and install.",
            pinned_to_vulnerable.join(", ")
        )));
    }

    // Nothing was rewritten, so the manifest is only re-resolved to prove the
    // current ranges still admit a clean graph.
    // Không gì bị ghi đè: manifest chỉ được resolve lại để chứng minh range
    // hiện tại vẫn cho ra graph sạch.
    let graph = resolve_fn(manifest.clone()).await?;

    // A re-resolve is not proof of a fix: `^1.0.0` still admits the vulnerable
    // 1.0.0. Refuse instead of writing a lockfile that re-audits dirty.
    // Resolve lại không chứng minh đã vá: `^1.0.0` vẫn chứa bản dính 1.0.0.
    // Từ chối thay vì ghi lockfile còn dính lỗ hổng.
    let still_vulnerable: Vec<String> = graph
        .packages
        .iter()
        .filter(|pkg| vulnerable.iter().any(|id| id == &pkg.id))
        .map(|pkg| pkg.id.to_string())
        .collect();
    if !still_vulnerable.is_empty() {
        return Err(MgError::Other(format!(
            "audit --fix: the current range still resolves the vulnerable release {}. \
             Widen the manifest range to exclude it, then install. \
             MagiCore will not rewrite your constraint to '*' to silence an advisory.",
            still_vulnerable.join(", ")
        )));
    }

    write_web_lockfile_with_state(project_root, &graph, "fixed")?;
    Ok(bumps.len())
}
