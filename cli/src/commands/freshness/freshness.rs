//! Cross-ecosystem lock freshness preflight. This stable facade re-exports the existing API; the coordinator preserves manifest order and the lockless union-check exception. — Preflight nhất quán lock xuyên ecosystem. Facade ổn định re-export API hiện có; coordinator giữ thứ tự manifest và ngoại lệ lockless của kiểm tra hợp.

use anyhow::Context;
use std::path::Path;

use anyhow::Result;

use super::cargo::declared_cargo_deps;
use super::flutter::check_flutter_manifest_freshness;
use super::go::declared_go_deps;
use super::manifest_file::check_manifest_file;
use super::npm::declared_npm_deps;
use super::python::declared_python_deps;

/// Union pre-flight across every manifest the test runner understands
/// (Spec rev4 §8): each present manifest must match the lock. Manifests
/// without a verified lock-name shape (maven/nuget/gradle) stay out until
/// per-lane lock evidence exists.
/// (Pre-flight hợp mọi manifest mà test runner hiểu (Spec rev4 §8): mỗi
/// manifest hiện diện phải khớp lock. Manifest chưa xác minh dạng tên lock
/// (maven/nuget/gradle) đứng ngoài cho đến khi có evidence từng lane.)
pub fn check_any_freshness(project_root: &Path) -> Result<()> {
    // Absent lock means nothing to be consistent with (lockless projects,
    // e.g. optimizer fixtures, keep working); per-manifest `check_*`
    // functions used by dev entries still fail closed on a missing lock.
    // (Chưa có lock nghĩa là không có gì để nhất quán (project chưa lock,
    // vd fixture optimizer, vẫn chạy); hàm `check_*` từng manifest mà entry
    // dev dùng vẫn fail-closed khi thiếu lock.)
    if !project_root.join("mgc.lock").exists() {
        return Ok(());
    }
    // go.mod `replace` rewrites requirements the lock may not contain under
    // the original path — skip the go projection then (warn, don't guess).
    // (`replace` trong go.mod viết lại requirement mà lock có thể không chứa
    // dưới path gốc — khi đó bỏ qua projection go (cảnh báo, không đoán).)
    let go_path = project_root.join("go.mod");
    let go_has_replace = go_path.exists()
        && std::fs::read_to_string(&go_path)
            .map(|content| {
                content
                    .lines()
                    .any(|line| line.trim_start().starts_with("replace"))
            })
            .unwrap_or(false);
    if go_has_replace {
        mgc_ui::warning("go.mod has replace directives; skipping go freshness projection");
    }
    check_manifest_file(
        project_root,
        "package.json",
        false,
        "web",
        mgc_lockfile::EcosystemTag::Web,
        |content| {
            let value = serde_json::from_str(content).context("package.json is not valid JSON")?;
            Ok(declared_npm_deps(&value))
        },
    )?;
    check_manifest_file(
        project_root,
        "pyproject.toml",
        false,
        "ai",
        mgc_lockfile::EcosystemTag::Python,
        |content| Ok(declared_python_deps(content)),
    )?;
    check_flutter_manifest_freshness(project_root, false)?;
    check_manifest_file(
        project_root,
        "Cargo.toml",
        false,
        "game",
        mgc_lockfile::EcosystemTag::Rust,
        |content| Ok(declared_cargo_deps(content)),
    )?;
    if !go_has_replace {
        check_manifest_file(
            project_root,
            "go.mod",
            false,
            "lib",
            mgc_lockfile::EcosystemTag::Go,
            |content| Ok(declared_go_deps(content)),
        )?;
    }
    Ok(())
}
