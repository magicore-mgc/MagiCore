#![allow(clippy::unwrap_used)]

use super::{DEFAULT_WORKFLOW_IDENTITY_PREFIX, workflow_identity_allowed};

// Keeps the constant referenced so the test breaks if the pinned prefix moves.
// (Giữ tham chiếu hằng để test fail nếu prefix pin bị đổi.)
const _: &str = DEFAULT_WORKFLOW_IDENTITY_PREFIX;

/// SEC-05: with no allowlist configured, only this repository's own release
/// workflows may attest. Another account's forge must not qualify.
/// (SEC-05: không cấu hình allowlist thì chỉ workflow release của chính repo
/// này được chứng thực. Forge của tài khoản khác không được hợp lệ.)
#[test]
fn default_identity_accepts_only_this_repository_release_workflows() {
    let none: Vec<String> = Vec::new();
    for allowed in [
        "https://github.com/mingd-153/MagiCore/.github/workflows/release.yml",
        "https://github.com/mingd-153/MagiCore/.github/workflows/release-publish-github.yml",
    ] {
        assert!(
            workflow_identity_allowed(allowed, &none),
            "{allowed} must be accepted"
        );
    }
    for denied in [
        "https://github.com/attacker/MagiCore/.github/workflows/release.yml",
        "https://github.com/mingd-153/other/.github/workflows/release.yml",
        "https://github.com/mingd-153/MagiCore/.github/workflows/ci.yml",
        "https://gitlab.com/mingd-153/MagiCore/.github/workflows/release.yml",
    ] {
        assert!(
            !workflow_identity_allowed(denied, &none),
            "{denied} must be refused"
        );
    }
}

/// An explicit allowlist is authoritative in both directions: it can add a
/// workflow outside this repository and it can remove one inside it.
/// (Allowlist tường minh quyết định cả hai chiều: thêm workflow ngoài repo và
/// loại một workflow trong repo.)
#[test]
fn explicit_allowlist_overrides_the_default() {
    let allow =
        vec!["https://github.com/fork-owner/MagiCore/.github/workflows/release.yml".to_string()];
    assert!(workflow_identity_allowed(&allow[0], &allow));
    assert!(!workflow_identity_allowed(
        "https://github.com/mingd-153/MagiCore/.github/workflows/release.yml",
        &allow
    ));
}
