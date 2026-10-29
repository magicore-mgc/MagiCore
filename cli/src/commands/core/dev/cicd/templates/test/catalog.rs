use crate::commands::core::dev::cicd::templates::CIRCLE_TEMPLATE;
use crate::commands::core::dev::cicd::templates::GITLAB_TEMPLATE;
use crate::commands::core::dev::cicd::templates::MGC_INSTALLER_SHA;
use crate::commands::core::dev::cicd::templates::MGC_INSTALLER_SHA256;
use crate::commands::core::dev::cicd::templates::MGC_RELEASE_TAG;
use crate::commands::core::dev::cicd::templates::WORKFLOW_TEMPLATE;

#[test]
fn ci_templates_cover_all_providers() {
    // Checkout must be pinned by commit SHA — a mutable tag can be hijacked.
    // Checkout phải ghim theo commit SHA — tag mutable có thể bị chiếm.
    assert!(
        WORKFLOW_TEMPLATE.contains("actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1")
    );
    assert!(!WORKFLOW_TEMPLATE.contains("checkout@v"));
    // Installer must come from a release tag, not a mutable branch.
    // Installer phải lấy từ release tag, không phải branch mutable.
    assert!(WORKFLOW_TEMPLATE.contains("--version {tag}"));
    assert!(!WORKFLOW_TEMPLATE.contains("--branch"));
    assert!(GITLAB_TEMPLATE.contains("mgc verify"));
    assert!(GITLAB_TEMPLATE.contains("stages:"));
    assert!(GITLAB_TEMPLATE.contains("--version {tag}"));
    assert!(CIRCLE_TEMPLATE.contains("version: 2.1"));
    assert!(CIRCLE_TEMPLATE.contains("install-from-gh.sh"));
    assert!(CIRCLE_TEMPLATE.contains("--version {tag}"));
}

#[test]
fn generated_ci_release_tag_matches_this_mgc_build() {
    // Generated workflows must install the exact MagiCore build that emitted
    // them; a stale manually copied tag silently pins old behavior.
    // (Workflow sinh ra phải cài đúng phiên bản MagiCore đang chạy; tag chép
    // tay bị cũ sẽ âm thầm ghim hành vi lỗi thời.)
    assert_eq!(MGC_RELEASE_TAG, format!("v{}", env!("CARGO_PKG_VERSION")));
}

#[test]
fn generated_ci_installer_pin_matches_current_release_contract() {
    // The embedded digest describes the immutable installer commit, not a
    // possibly edited working-tree copy. The CI contract checks the digest
    // against `git show <pin>:scripts/install-from-gh.sh`.
    // (Digest nhúng thuộc installer commit bất biến, không phải bản đang
    // sửa trong worktree; CI contract đối chiếu trực tiếp Git object.)
    const TRACKED_INSTALLER: &[u8] =
        include_bytes!("../../../../../../../../scripts/install/install-from-gh.sh");
    let installer = std::str::from_utf8(TRACKED_INSTALLER).expect("installer is UTF-8");
    assert!(installer.contains("OS_LABEL=\"macos\""));
    assert!(installer.contains("EXT=\"zip\""));
}

/// The pin must name a commit that CONTAINS the installer, and the digest must
/// be that file's bytes at that commit.
///
/// The previous pin named a commit from before the installer moved to
/// `scripts/install/`, so every generated pipeline downloaded a 404 as its first
/// step while the constant looked immutable. Asserting the pin against another
/// literal cannot catch that; asking git can.
/// (Pin phải trỏ commit THỰC SỰ chứa installer, và digest phải là byte của file
/// tại commit đó. Pin cũ trỏ commit trước khi installer chuyển sang
/// `scripts/install/`, nên mọi pipeline sinh ra nhận 404 ngay bước đầu dù hằng
/// số trông bất biến. So pin với một literal khác không bắt được lỗi đó; hỏi git
/// thì bắt được.)
#[test]
fn installer_pin_resolves_to_a_commit_that_contains_the_installer() {
    let Some(repo_root) = repo_root() else {
        eprintln!("skipping: not inside a git checkout");
        return;
    };
    let installer_path = "scripts/install/install-from-gh.sh";

    let blob = std::process::Command::new("git")
        .arg("-C")
        .arg(&repo_root)
        .args(["show", &format!("{MGC_INSTALLER_SHA}:{installer_path}")])
        .output()
        .expect("run git show");
    assert!(
        blob.status.success(),
        "MGC_INSTALLER_SHA {MGC_INSTALLER_SHA} does not contain {installer_path}; \
         generated pipelines would 404 on their first step: {}",
        String::from_utf8_lossy(&blob.stderr)
    );

    let digest = sha256_hex(&blob.stdout);
    assert_eq!(
        digest, MGC_INSTALLER_SHA256,
        "MGC_INSTALLER_SHA256 does not match {installer_path} at {MGC_INSTALLER_SHA}"
    );
}

/// Path of the repository root, or `None` when this is not a git checkout.
/// (Đường dẫn repo root, hoặc `None` khi không phải git checkout.)
fn repo_root() -> Option<std::path::PathBuf> {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(manifest)
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let path = String::from_utf8(out.stdout).ok()?;
    Some(std::path::PathBuf::from(path.trim()))
}

/// SHA-256 of `bytes`, lowercase hex — the form `shasum -a 256` prints and the
/// form the generated pipelines verify with.
/// (SHA-256 của `bytes` dạng hex thường — đúng dạng `shasum -a 256` in ra và
/// pipeline sinh ra dùng để verify.)
fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}
