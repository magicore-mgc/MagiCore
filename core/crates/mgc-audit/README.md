# mgc-audit

mgc-audit is the shared security-audit engine used by MagiCore adapters. It owns audit planning, scanner execution, report aggregation, and output contracts. Adapter-specific project discovery remains with the adapter that owns that ecosystem.

## Boundaries

- src/aggregate/ combines reports from every planned scanner under the existing strictest-status policy.
- src/contract.rs, src/engine.rs, and src/scanners/ own the plan and scanner execution APIs.
- mgc-types::adapter::{AuditReport, ScannerStatus} remain shared report contracts.
- The aggregate function is infallible and returns AuditReport; no local error type is needed.
- The aggregate_test Cargo target is beside the aggregate owner at src/aggregate/test/aggregate.rs; other crate-level scanner tests remain under tests/.

## Bản tiếng Việt

mgc-audit là engine audit bảo mật dùng chung cho các adapter MagiCore. Crate sở hữu kế hoạch audit, chạy scanner, tổng hợp report và contract đầu ra. Adapter tiếp tục sở hữu việc nhận diện dự án đặc thù của hệ sinh thái.

src/aggregate/ tổng hợp report từ mọi scanner trong plan theo policy trạng thái nghiêm ngặt nhất hiện có. AuditReport và ScannerStatus vẫn là contract dùng chung của mgc-types. Hàm aggregate không phát sinh lỗi và trả AuditReport, nên không tạo error.rs giả. Cargo target aggregate_test đặt cạnh owner tại src/aggregate/test/aggregate.rs; test scanner cấp crate khác vẫn ở tests/.
