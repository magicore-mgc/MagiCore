//! Hardware core audit — dispatch to the adapter's real scanner via the
//! shared pipeline (detect → adapter → scanner → typed report → exit
//! contract). Template packages carry no dependency graph, so the scanner
//! reports honestly Unavailable: strict CI fails (P0-7), local exits 0
//! only with a loud UNVERIFIED warning.
//! Audit core Hardware — dispatch qua pipeline chung; template package
//! không có dependency graph nên scanner trung thực Unavailable: strict CI
//! fail (P0-7), local chỉ exit 0 kèm cảnh báo UNVERIFIED rõ ràng.

use super::{OutputFormat, StrictMode, finish_and_print, run_adapter_audit};
use crate::context::ProjectContext;

pub(crate) async fn audit(
    ctx: &ProjectContext,
    fmt: OutputFormat,
    strict: StrictMode,
) -> anyhow::Result<()> {
    let report = run_adapter_audit(ctx).await?;

    // The package listing is human context — machine formats skip it and
    // print ONLY their structured payload.
    // Danh sách package là ngữ cảnh cho người — format máy bỏ qua và chỉ
    // in payload cấu trúc của mình.
    if !fmt.is_machine() {
        // The listing is supplementary context, not the audit result. A `list`
        // failure used to abort the whole command with `hardware core does not
        // support 'list' yet`, so the user asked for an audit and got an error
        // about a different capability — while the audit report they wanted had
        // already been produced.
        // (Danh sách là ngữ cảnh phụ, không phải kết quả audit. Lỗi `list` từng
        // hủy cả lệnh với thông điệp về `list`, dù report audit đã có sẵn.)
        match ctx.adapter().list(ctx.root()).await {
            Ok(pkgs) => {
                println!("  ℹ   hardware packages: {}", pkgs.len());
                for p in &pkgs {
                    println!("      {}@{}", p.id.name().as_str(), p.id.version());
                }
            }
            Err(_) => {
                // The adapter's `list` guidance talks about `mgc optimizer` —
                // irrelevant to someone who asked for an audit. Repeating it here
                // sends them to the wrong command, so the note names only what
                // matters: the listing is skipped, the verdict is not.
                // (Guidance của `list` nói về `mgc optimizer` — không liên quan với
                // người hỏi audit. Lặp lại sẽ chỉ sang lệnh sai, nên chỉ nêu điều
                // cần biết: bỏ phần danh sách, kết luận không đổi.)
                println!(
                    "  ℹ   hardware package listing skipped (this core has no package inventory); the audit verdict below is unaffected"
                );
            }
        }
    }

    // Shared fail-closed finisher — no more silent exit-0 on unavailable.
    // Finisher fail-closed chung — hết exit 0 âm thầm khi unavailable.
    finish_and_print("hardware", &report, strict, fmt).await
}
