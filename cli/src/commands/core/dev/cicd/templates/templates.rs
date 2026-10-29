/// Pin generated CI to the exact version of the MagiCore binary generating it.
/// (Workflow mới ghim đúng version binary MagiCore đang sinh ra nó.)
pub(in crate::commands::core::dev::cicd) const MGC_RELEASE_TAG: &str =
    concat!("v", env!("CARGO_PKG_VERSION"));

/// Install source used by generated CI templates: the latest GitHub Release
/// installer downloaded from an IMMUTABLE commit SHA — never from the
/// mutable `main` branch (P0-E, 2026-09-16 supply-chain gate). Update the
/// SHA to the last commit that touched `scripts/install/install-from-gh.sh`
/// whenever MGC_RELEASE_TAG moves.
///
/// This value MUST be the commit that actually contains the installer at
/// `scripts/install/install-from-gh.sh`. An earlier pin named a commit that
/// predates that file's move, so every generated pipeline fetched a 404 as its
/// first step — the pin looked immutable and was simply wrong. The test in
/// `test/installer_pin.rs` now derives the expected values from git, so a stale
/// pin fails instead of shipping a broken pipeline.
/// (P0-E: installer trong template CI được tải từ commit SHA BẤT BIẾN —
/// không bao giờ từ branch `main` mutable. Giá trị này PHẢI là commit thực sự
/// chứa installer tại `scripts/install/install-from-gh.sh`. Pin cũ trỏ commit
/// trước khi file được di chuyển, nên mọi pipeline sinh ra nhận 404 ngay bước
/// đầu — pin trông bất biến mà sai. Test `test/installer_pin.rs` nay suy ra giá
/// trị đúng từ git nên pin cũ sẽ fail thay vì ship pipeline hỏng.)
pub(in crate::commands::core::dev::cicd) const MGC_INSTALLER_SHA: &str =
    "51acbbb942d48407af82f5282fed0ffd5038516d";

/// Embedded SHA-256 of `scripts/install/install-from-gh.sh` at `MGC_INSTALLER_SHA`.
/// Contract (P0-E/T0.5):
/// - empty → the generated pipeline prints a loud WARNING and still runs
///   (no break for existing pipelines);
/// - a `.sha256` file published next to the installer is verified whenever
///   present (mismatch fails the pipeline);
/// - when set, a checksum mismatch FAILS the pipeline (fail-closed).
///
/// Equal to `sha256(git show <MGC_INSTALLER_SHA>:scripts/install/install-from-gh.sh)`.
///
/// (P0-E/T0.5: SHA-256 nhúng của installer tại `MGC_INSTALLER_SHA`. Rỗng →
/// pipeline in WARNING rõ ràng rồi vẫn chạy; file `.sha256` cạnh installer
/// được verify khi có (lệch → fail); khi đặt giá trị → lệch checksum FAIL.
/// Giá trị bằng `sha256(git show <MGC_INSTALLER_SHA>:scripts/…/install-from-gh.sh)`.)
pub(in crate::commands::core::dev::cicd) const MGC_INSTALLER_SHA256: &str =
    "903f620f989e0314007ed8258523c42a39d05c3dfadba8b49bb91603b45bb9db";

/// Render a CI template: release tag + immutable installer pin (P0-E).
/// Every placeholder substitution lives in ONE helper so a half-rendered
/// template (e.g. a missing SHA pin) can never reach disk.
/// (Render template CI: tag release + ghim installer bất biến (P0-E). Mọi
/// placeholder thay tại MỘT helper duy nhất nên template render thiếu
/// (vd thiếu SHA pin) không bao giờ chạm đĩa.)
pub(in crate::commands::core::dev::cicd) fn render_ci_template(
    template: &str,
    name: &str,
) -> String {
    render_ci_template_with_repo(template, name, "{repo_url}")
}

/// Render a template that also needs the project's repository URL.
/// Render template cần thêm URL repository của project.
pub(in crate::commands::core::dev::cicd) fn render_ci_template_with_repo(
    template: &str,
    name: &str,
    repo_url: &str,
) -> String {
    // SEC-10: an unpinned installer digest is not a warning, it is a missing
    // release: a template must never be written with an empty pin, because the
    // pipeline would then install whatever the tag points at today.
    // (SEC-10: digest installer không pin không phải cảnh báo mà là release
    // thiếu: template không bao giờ được ghi với pin rỗng, vì pipeline sẽ cài
    // bất cứ thứ gì tag trỏ tới hôm đó.)
    debug_assert!(
        !MGC_INSTALLER_SHA256.is_empty(),
        "MGC_INSTALLER_SHA256 must be pinned in the release build"
    );
    template
        .replace("{repo_url}", repo_url)
        .replace("{name}", name)
        .replace("{tag}", MGC_RELEASE_TAG)
        .replace("{installer_sha}", MGC_INSTALLER_SHA)
        .replace("{installer_sha256}", MGC_INSTALLER_SHA256)
}

/// Refuse to generate when the release pin is missing (SEC-10).
/// Từ chối sinh khi thiếu pin release (SEC-10).
pub(in crate::commands::core::dev::cicd) fn ensure_release_pin() -> anyhow::Result<()> {
    if MGC_INSTALLER_SHA256.trim().is_empty() || MGC_INSTALLER_SHA.trim().is_empty() {
        anyhow::bail!(
            "refusing to generate CI: the installer pin is empty in this build \
             (MGC_INSTALLER_SHA/MGC_INSTALLER_SHA256); rebuild from a released source tree"
        );
    }
    Ok(())
}
