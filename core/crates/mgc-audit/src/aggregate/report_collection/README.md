# Report collection owner

report_collection performs the single ordered pass over scanner reports. It collects package totals, vulnerability rows, scanned/skipped counts, reasons, and the state facts required by status resolution. It intentionally retains findings from Partial reports.

The owner shares the existing AuditReport and ScannerStatus contracts from mgc-types. Its ReportSummary is private to the aggregate tree; it adds no public API and has no error boundary. Direct owner tests live under test/.

## Bản tiếng Việt

report_collection duyệt các report scanner đúng một lượt theo thứ tự đầu vào. Owner gom tổng package, vulnerability row, số đã scan/bỏ qua, lý do và các cờ trạng thái cho bộ phân xử. Finding từ report Partial được giữ lại.

Owner dùng contract AuditReport và ScannerStatus có sẵn của mgc-types. ReportSummary chỉ dùng bên trong cây aggregate, không mở rộng API và không phát sinh error boundary. Test trực tiếp của owner nằm trong test/.
