# Audit aggregation owner

The public coordinator preserves the aggregate_reports contract and delegates two distinct jobs:

- report_collection performs one ordered pass and collects package totals, findings, skipped reasons, and scanner-state facts.
- status_resolution applies the established strictest-status decision table and builds the final verdict.

mod.rs is wiring only. Public routes remain mgc_audit::aggregate_reports and mgc_audit::aggregate::aggregate_reports. The shared AuditReport and ScannerStatus contracts stay in mgc-types. ReportSummary is an internal handoff value scoped to this owner tree. Aggregation has no local fallible operation, so no error type is fabricated.

The existing aggregate_test integration target remains at test/aggregate.rs. The report_collection and status_resolution folders hold their own unit-test source under their local test/ folders.

## Bản tiếng Việt

Coordinator công khai giữ contract aggregate_reports và giao hai trách nhiệm riêng:

- report_collection duyệt report một lượt theo thứ tự, gom tổng package, finding, lý do bỏ qua và các dữ kiện trạng thái scanner.
- status_resolution áp dụng bảng ưu tiên trạng thái hiện có và dựng verdict cuối.

mod.rs chỉ wiring. Hai route công khai vẫn là mgc_audit::aggregate_reports và mgc_audit::aggregate::aggregate_reports. Contract AuditReport và ScannerStatus vẫn ở mgc-types. ReportSummary là dữ liệu trung chuyển nội bộ trong owner này. Aggregation không có thao tác phát sinh lỗi nên không tạo error type giả.

Integration target aggregate_test cũ vẫn ở test/aggregate.rs. Hai folder report_collection và status_resolution có unit-test source riêng trong test/ gần owner.
