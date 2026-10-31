use anyhow::Result;

/// `mgc verify` — chạy chain theo adapter: audit (web P1) → test → build (07 §4).
/// 1 bước fail → dừng, báo rõ project (workspace recursive P2 — chỉ cwd P1).
///
/// Fail-closed contract (Tech Lead P0-3, 2026-09-12):
/// - unknown step trong chain → ERROR (không skip im lặng);
/// - audit luôn chạy strict trong CI (MGC_AUDIT_STRICT=1) — exit 2 nếu UNVERIFIED;
/// - không bước nào được bỏ qua rồi vẫn in "Verify chain OK".
///
/// Hợp đồng fail-closed: step lạ trong chain → lỗi; audit strict trong CI;
/// không được bỏ step rồi vẫn báo thành công.
pub async fn verify() -> Result<()> {
    let root = std::env::current_dir().map_err(|e| crate::error::cwd_deleted(&e))?;
    mgc_ui::info(&format!("[verify] project: {}", root.display()));

    let chain = super::super::project_config::verify_chain(&root)?;
    mgc_ui::info(&format!("[verify] chain: {}", chain.join(" → ")));

    // Validate the whole chain BEFORE running anything — a typo'd step name
    // fails up front instead of being skipped mid-run.
    // Validate toàn bộ chain TRƯỚC khi chạy — tên step gõ sai fail ngay từ
    // đầu thay vì bị bỏ qua giữa chừng.
    const KNOWN_STEPS: [&str; 3] = ["audit", "test", "build"];
    for step in &chain {
        if !KNOWN_STEPS.contains(&step.as_str()) {
            return Err(crate::error::cicd_verify_unknown_step(step));
        }
    }
    if chain.is_empty() {
        return Err(crate::error::cicd_verify_empty_chain());
    }

    let core = mgc_config::project::ProjectConfig::load(&root)
        .ok()
        .flatten()
        .map(|cfg| cfg.ecosystem)
        .unwrap_or_default();

    // Note: audit strictness is enforced inside the audit runner — CI
    // environments (CI=true) default to strict, so an UNVERIFIED audit can
    // never pass this chain silently. No env mutation needed here.
    // Ghi chú: strict do audit runner tự thực thi — môi trường CI (CI=true)
    // mặc định strict nên UNVERIFIED không thể lọt chain. Không cần set env.

    for step in &chain {
        match step.as_str() {
            "audit" => {
                // A verify chain is a release gate: incomplete scanner coverage fails locally and in CI.
                // Chuỗi verify là cổng phát hành: scanner thiếu coverage phải lỗi cả local lẫn CI.
                crate::commands::audit::run_strict(None, false, None).await?;
            }
            "test" => super::step_runner::run_test_step(&root, &core).await?,
            "build" => {
                super::build_step::ensure_build_step_supported(&core)?;
                crate::commands::build::run(None, None, None, None, None, false, false).await?;
            }
            // Unreachable (chain validated above) — kept fail-closed anyway.
            // Không thể tới đây (chain đã validate) — vẫn giữ fail-closed.
            other => return Err(crate::error::cicd_verify_unknown_step(other)),
        }
    }
    mgc_ui::success("Verify chain OK");
    Ok(())
}

#[cfg(test)]
#[path = "test/verify.rs"]
mod tests;
