//! Applies trust policy to scanned packages and renders stable CLI output.
//! Áp dụng trust policy lên package đã quét và in output CLI ổn định.

use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

pub(in crate::commands::trust::pending) fn render_report(
    scripted: HashSet<(String, String)>,
    db_policies: &HashMap<String, String>,
    file_policy: Option<&mgc_config::project::ScriptsPolicy>,
) {
    let summary = classify_scripted_packages(scripted, db_policies, file_policy);
    let stdout = std::io::stdout();
    let mut output = stdout.lock();
    if let Err(error) = write_report_lines(&summary, &mut output) {
        panic!("failed printing to stdout: {error}");
    }
}

fn write_report_lines(summary: &PendingSummary, output: &mut impl Write) -> io::Result<()> {
    if !summary.denied.is_empty() {
        writeln!(output, "Denied (scripts blocked):")?;
        for id in &summary.denied {
            writeln!(output, "  ✗ {id}")?;
        }
    }
    if !summary.approved.is_empty() {
        writeln!(output, "Approved (scripts will run):")?;
        for id in &summary.approved {
            writeln!(output, "  ✓ {id}")?;
        }
    }
    if summary.pending.is_empty() {
        writeln!(
            output,
            "No pending packages — every installed lifecycle script has a policy."
        )?;
    } else {
        writeln!(
            output,
            "Pending review (scripts SKIPPED on install until approved):"
        )?;
        for id in &summary.pending {
            writeln!(output, "  ? {id} — approve with: mgc trust approve {id}")?;
        }
    }
    Ok(())
}

/// Classifies each scripted package once while preserving deterministic output order.
/// Phân loại mỗi package có script đúng một lần và giữ thứ tự output xác định.
fn classify_scripted_packages(
    scripted: HashSet<(String, String)>,
    db_policies: &HashMap<String, String>,
    file_policy: Option<&mgc_config::project::ScriptsPolicy>,
) -> PendingSummary {
    let mut summary = PendingSummary::default();
    let mut scripted: Vec<(String, String)> = scripted.into_iter().collect();
    scripted.sort();
    for (name, version) in scripted {
        let id = format!("{name}@{version}");
        let db = db_policies
            .get(&id)
            .or_else(|| db_policies.get(&name))
            .map(String::as_str);
        // ONE merged decision (mgc-config::decide_scripts — shared with
        // the install gate): deny wins from either source, then allow.
        // Blanket is false here: anything undecided is pending review,
        // never silently allowed.
        // (Một quyết định gộp duy nhất — chưa quyết thì pending.)
        match mgc_config::project::decide_scripts(&name, &version, file_policy, db, false) {
            mgc_config::project::ScriptVerdict::Deny(_) => summary.denied.push(id),
            mgc_config::project::ScriptVerdict::Allow(_) => summary.approved.push(id),
            mgc_config::project::ScriptVerdict::Undecided => summary.pending.push(id),
        }
    }
    summary
}

/// Sorted results of applying the shared script-policy decision.
/// Kết quả đã sắp xếp sau khi áp dụng quyết định script-policy dùng chung.
#[derive(Debug, Default, Eq, PartialEq)]
struct PendingSummary {
    approved: Vec<String>,
    denied: Vec<String>,
    pending: Vec<String>,
}

#[cfg(test)]
#[path = "test/report.rs"]
mod tests;
