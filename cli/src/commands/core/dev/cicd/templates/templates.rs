/// Pin generated CI to the exact version of the MagiCore binary generating it.
/// (Workflow mới ghim đúng version binary MagiCore đang sinh ra nó.)
pub(in crate::commands::core::dev::cicd) const MGC_RELEASE_TAG: &str =
    concat!("v", env!("CARGO_PKG_VERSION"));

/// Install source used by generated CI templates: the latest GitHub Release
/// installer downloaded from an IMMUTABLE commit SHA — never from the
/// mutable `main` branch (P0-E, 2026-09-16 supply-chain gate). Update the
/// SHA to the last commit that touched `scripts/install-from-gh.sh`
/// whenever MGC_RELEASE_TAG moves.
/// (P0-E: installer trong template CI được tải từ commit SHA BẤT BIẾN —
/// không bao giờ từ branch `main` mutable. Cập nhật SHA theo commit cuối
/// chạm `scripts/install-from-gh.sh` mỗi khi MGC_RELEASE_TAG dịch chuyển.)
pub(in crate::commands::core::dev::cicd) const MGC_INSTALLER_SHA: &str =
    "80a383763377413c4a50d8de3bd815a0f8f01284";

/// Embedded SHA-256 of `scripts/install-from-gh.sh` at `MGC_INSTALLER_SHA`.
/// Contract (P0-E/T0.5):
/// - empty → the generated pipeline prints a loud WARNING and still runs
///   (no break for existing pipelines);
/// - a `.sha256` file published next to the installer is verified whenever
///   present (mismatch fails the pipeline);
/// - when set, a checksum mismatch FAILS the pipeline (fail-closed).
///
/// (P0-E/T0.5: SHA-256 nhúng của installer tại `MGC_INSTALLER_SHA`. Rỗng →
/// pipeline in WARNING rõ ràng rồi vẫn chạy; file `.sha256` cạnh installer
/// được verify khi có (lệch → fail); khi đặt giá trị → lệch checksum FAIL.)
pub(in crate::commands::core::dev::cicd) const MGC_INSTALLER_SHA256: &str =
    "2e722bbebdccad1f00719da21f56963d44ff519d3868538f9abdc2e03a27ef56";

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
    template
        .replace("{name}", name)
        .replace("{tag}", MGC_RELEASE_TAG)
        .replace("{installer_sha}", MGC_INSTALLER_SHA)
        .replace("{installer_sha256}", MGC_INSTALLER_SHA256)
}
