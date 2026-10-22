use crate::commands::core::dev::cicd::templates::CIRCLE_TEMPLATE;
use crate::commands::core::dev::cicd::templates::GITLAB_TEMPLATE;
use crate::commands::core::dev::cicd::templates::MGC_INSTALLER_SHA;
use crate::commands::core::dev::cicd::templates::MGC_INSTALLER_SHA256;
use crate::commands::core::dev::cicd::templates::MGC_RELEASE_TAG;
use crate::commands::core::dev::cicd::templates::WORKFLOW_TEMPLATE;
use crate::commands::core::dev::cicd::templates::render_ci_template;

#[test]
fn ci_templates_install_from_immutable_sha_never_main() {
    // P0-E supply-chain gate (2026-09-16): the generated installer URL
    // must pin an immutable commit SHA — mutable `/main/` may not appear
    // anywhere; the script is downloaded to a FILE (never `curl | bash`);
    // companion/embedded checksum verification logic must be present;
    // with an empty embedded checksum the pipeline must WARN, not break.
    // (Cổng supply-chain P0-E: URL installer phải ghim commit SHA bất
    // biến — không được có `/main/` mutable ở đâu cả; script tải về FILE
    // (không bao giờ `curl | bash`); logic verify checksum companion/nhúng
    // phải có; checksum nhúng rỗng thì pipeline phải WARNING, không vỡ.)
    assert_eq!(MGC_INSTALLER_SHA.len(), 40, "pin must be a full commit SHA");
    assert!(
        !MGC_INSTALLER_SHA.chars().any(|c| !c.is_ascii_hexdigit()),
        "pin must be hex-only"
    );
    for (rendered, label) in [
        (
            render_ci_template(WORKFLOW_TEMPLATE, "CI"),
            "github workflow",
        ),
        (render_ci_template(GITLAB_TEMPLATE, ""), "gitlab ci"),
        (render_ci_template(CIRCLE_TEMPLATE, ""), "circle ci"),
    ] {
        assert_eq!(
            rendered.matches("/main/").count(),
            0,
            "{label}: installer URL must never reference mutable main"
        );
        assert!(
            rendered.contains(MGC_INSTALLER_SHA),
            "{label}: installer URL must pin the immutable commit SHA"
        );
        assert!(
            !rendered.contains("| bash"),
            "{label}: installer must never be piped straight into bash"
        );
        assert!(
            rendered.contains("-o \"$installer\""),
            "{label}: installer must be downloaded to a file first"
        );
        assert!(
            rendered.contains("sha256sum -c"),
            "{label}: companion .sha256 verification must be present"
        );
        assert!(
            rendered.contains("WARNING: installer integrity NOT pinned"),
            "{label}: empty embedded checksum must print a loud warning and proceed"
        );
        assert!(
            rendered.contains("--version {tag}".replace("{tag}", MGC_RELEASE_TAG).as_str()),
            "{label}: release-tag install flag must survive rendering"
        );
    }
}

#[test]
fn installer_checksum_is_pinned_hex() {
    // T0.5: the embedded checksum must be a real SHA-256 (64 lowercase
    // hex), never empty — empty means warning-only mode.
    // (Checksum nhúng phải là SHA-256 thật, không bao giờ rỗng.)
    assert_eq!(MGC_INSTALLER_SHA256.len(), 64);
    assert!(
        MGC_INSTALLER_SHA256
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    );
}

#[test]
fn rendered_templates_verify_checksum_fail_closed() {
    // T0.5: every generated template must substitute the pins, verify
    // checksums with fail-closed semantics, and never fetch from a
    // mutable branch.
    for template in [WORKFLOW_TEMPLATE, GITLAB_TEMPLATE, CIRCLE_TEMPLATE] {
        let rendered = render_ci_template(template, "CI");
        assert!(
            !rendered.contains("{installer_sha256}"),
            "sha256 pin must be substituted"
        );
        assert!(
            !rendered.contains("{installer_sha}"),
            "commit SHA pin must be substituted"
        );
        assert!(
            rendered.contains(MGC_INSTALLER_SHA256),
            "embedded checksum must reach the pipeline"
        );
        assert!(
            rendered.contains("sha256sum -c"),
            "pipeline must verify checksums"
        );
        assert!(
            !rendered.contains("raw.githubusercontent.com/mingd-153/MagiCore/main/"),
            "generated pipeline must never fetch from mutable main"
        );
    }
}
