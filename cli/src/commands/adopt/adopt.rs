//! `mgc adopt` — take ownership of a project another tool created.
//! `mgc adopt` — nhận sở hữu project do công cụ khác tạo.
//!
//! Cores without a CLI initializer (`unity`, `unreal`, GitHub Actions, Argo CD)
//! are created by their own tool or by hand. Adoption is the supported way to
//! bring such a project under MagiCore: it verifies the directory really looks
//! like that core's project, then writes the identity MagiCore owns.
//! Core không có initializer CLI được tạo bằng công cụ của nó hoặc bằng tay.
//! Adopt là đường hợp lệ để đưa project đó vào MagiCore: kiểm thư mục đúng là
//! project của core ấy, rồi ghi danh tính do MagiCore sở hữu.
//!
//! Adoption never invents project files. It refuses a directory that does not
//! carry the core's marker, because claiming a project that is not what it
//! looks like is exactly the mislabelling this codebase avoids.
//! Adopt không bao giờ tự tạo file project. Nó từ chối thư mục không mang
//! marker của core, vì nhận một project không đúng bản chất chính là dán nhãn
//! sai mà codebase này tránh.

use std::path::Path;

use anyhow::{Context, Result};
use mgc_config::project::ProjectConfig;

/// Marker files that prove a directory belongs to a core.
/// File marker chứng minh thư mục thuộc một core.
/// Join a marker's path segments onto `root` conceptually: the helper only
/// exists so the search reads the same for every marker length.
/// Nối các thành phần của marker vào `root`; helper chỉ để vòng lặp đọc giống
/// nhau với mọi độ dài marker.
fn segments_of<'a>(paths: &'a &'static [&'static str]) -> impl Iterator<Item = &'static str> + 'a {
    paths.iter().copied()
}

fn core_markers(core: &str) -> Option<&'static [&'static [&'static str]]> {
    Some(match core {
        "cicd" => &[&[".github", "workflows"], &["argocd"], &["charts"]],
        "clo" => &[
            &["cdk.json"],
            &["main.tf"],
            &["Pulumi.yaml"],
            &["serverless.yml"],
        ],
        "game" => &[
            &["project.godot"],
            &["ProjectSettings"],
            &["Assets"],
            &["Cargo.toml"],
        ],
        "iot" => &[
            &["platformio.ini"],
            &["west.yml"],
            &["sdkconfig"],
            &["Cargo.toml"],
        ],
        _ => return None,
    })
}

/// Adopt `root` as a `core` project, writing the identity MagiCore owns.
///
/// Refuses when the directory carries none of that core's markers, and refuses
/// to overwrite an existing `mgc.toml` so adoption can never silently replace
/// a project's recorded identity.
/// Nhận `root` làm project `core`, ghi danh tính MagiCore sở hữu. Từ chối khi
/// thư mục không có marker của core và không ghi đè `mgc.toml` đã có, để adopt
/// không bao giờ âm thầm thay danh tính đã ghi của project.
pub fn run(core: &str, root: &Path) -> Result<()> {
    let markers = core_markers(core).ok_or_else(|| {
        anyhow::anyhow!(
            "'{core}' has an initializer; use `mgc create-{core}` instead of adopting. \
             Adoption exists for cores whose project is created by the framework's own tool"
        )
    })?;

    if !root.is_dir() {
        anyhow::bail!("'{}' is not a directory", root.display());
    }

    let found = markers.iter().find(|paths| {
        let mut candidate = root.to_path_buf();
        for segment in segments_of(paths) {
            candidate.push(segment);
        }
        candidate.exists()
    });
    if found.is_none() {
        let expected: Vec<String> = markers.iter().map(|paths| paths.join("/")).collect();
        anyhow::bail!(
            "'{}' does not look like a {core} project: none of {} exists. \
             MagiCore will not record an identity the directory does not prove",
            root.display(),
            expected.join(", ")
        );
    }

    if root.join("mgc.toml").exists() {
        anyhow::bail!(
            "'{}' is already a MagiCore project (mgc.toml exists); \
             adoption would overwrite its recorded identity",
            root.display()
        );
    }

    ProjectConfig::write_core_marker_at(root, core)
        .with_context(|| format!("write core identity for '{}'", root.display()))?;

    let mut config = ProjectConfig::new(
        root.file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| "project".to_string()),
        "0.1.0".to_string(),
    );
    config.ecosystem = core.to_string();
    config
        .save(root)
        .with_context(|| format!("write mgc.toml for '{}'", root.display()))?;

    mgc_ui::success(&format!(
        "Adopted '{}' as a {core} project (marker {} matched). Run `mgc install` to start managing it.",
        root.display(),
        found.expect("checked above").join("/"),
    ));
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
