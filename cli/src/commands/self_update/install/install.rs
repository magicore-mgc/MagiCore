//! Replace the executable with rollback until version verification succeeds.
//! Thay binary và giữ rollback tới khi xác minh version thành công.

use super::probe_reports_version;
use crate::commands::self_update::error::SelfUpdateError;
use anyhow::Result;
/// Swap the running binary with the staged one, keeping a rollback
/// backup until the new binary proves itself: it must launch (exit 0)
/// AND report EXACTLY the requested version (`mgc <version>` token). A
/// wrong-version artifact is rolled back, never kept with the backup
/// deleted. Windows: a running exe cannot be overwritten, but it CAN be
/// renamed aside — so rename-then-move works on all three OSes.
/// (Thay binary, giữ backup tới khi probe khớp version chính xác.)
pub(in crate::commands) async fn swap_binary(
    staged: &std::path::Path,
    version_number: &str,
) -> Result<()> {
    let current =
        std::env::current_exe().map_err(|e| anyhow::anyhow!("locate running binary: {e}"))?;
    let backup = current.with_extension("mgc-backup");
    let _ = std::fs::remove_file(&backup);
    std::fs::rename(&current, &backup)
        .map_err(|e| anyhow::anyhow!("back up running binary: {e}"))?;
    if let Err(e) = std::fs::rename(staged, &current) {
        let _ = std::fs::rename(&backup, &current);
        return Err(anyhow::anyhow!("install new binary (rolled back): {e}"));
    }
    // Prove the new binary launches AND reports the requested version —
    // spawn by absolute path (project-binary guardrails: canonicalized
    // file). stdout_tail carries `--version` output for the exact check.
    // (Chứng minh binary mới chạy được VÀ đúng version yêu cầu.)
    let probe = mgc_exec::prelude::run_project_binary(
        &current,
        &["--version".to_string()],
        &mgc_exec::prelude::ExecOptions {
            ..Default::default()
        },
    );
    let rollback = |detail: String| {
        let _ = std::fs::rename(&backup, &current);
        SelfUpdateError::probe_failed(&detail)
    };
    let report = match probe {
        Ok(report) if report.exit_code == 0 => report,
        Ok(report) => {
            return Err(rollback(format!("exit code {}", report.exit_code)));
        }
        Err(e) => {
            return Err(rollback(format!("did not launch: {e}")));
        }
    };
    let expected = format!("mgc {version_number}");
    // The probe output must contain the requested version as a
    // whitespace-delimited token (`mgc 1.1.0-rc.10`): substring matching
    // would accept `1.1.0-rc.100`.
    // (Output probe phải chứa đúng version yêu cầu.)
    if !probe_reports_version(&report.stdout_tail, version_number) {
        return Err(rollback(format!(
            "reported {:?} instead of {expected}",
            report.stdout_tail.trim()
        )));
    }
    let _ = std::fs::remove_file(&backup);
    Ok(())
}
