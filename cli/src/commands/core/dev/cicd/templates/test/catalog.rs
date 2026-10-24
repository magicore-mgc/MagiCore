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
    assert_eq!(
        MGC_INSTALLER_SHA,
        "80a383763377413c4a50d8de3bd815a0f8f01284"
    );
    assert_eq!(
        MGC_INSTALLER_SHA256,
        "2e722bbebdccad1f00719da21f56963d44ff519d3868538f9abdc2e03a27ef56"
    );
    let installer = std::str::from_utf8(TRACKED_INSTALLER).expect("installer is UTF-8");
    assert!(installer.contains("OS_LABEL=\"macos\""));
    assert!(installer.contains("EXT=\"zip\""));
}
