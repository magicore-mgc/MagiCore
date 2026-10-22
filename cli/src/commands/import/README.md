# Legacy lockfile import / Import lockfile cũ

Owns the shared CLI migration from supported legacy lockfiles into `mgc.lock`. The public `commands::import::run` route and crate-visible merge helper remain stable. The coordinator keeps writer lock, pending-journal refusal, parse/report, snapshots, merge, signing decision, write, verification/rollback and final diagnostics in that order.

Sở hữu luồng CLI chung để chuyển lockfile cũ được hỗ trợ sang `mgc.lock`. Giữ route `commands::import::run` và helper merge trong crate. Coordinator giữ nguyên thứ tự khóa ghi, chặn journal chưa phục hồi, parse/báo cáo, snapshot, merge, quyết định ký, ghi, verify/rollback và chẩn đoán cuối.
