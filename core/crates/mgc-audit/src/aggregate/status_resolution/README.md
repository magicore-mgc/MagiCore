# Status resolution owner

status_resolution owns only the verdict decision table. It preserves failure over missing-tool priority, all-unsupported detection, Available/Partial resolution, the empty-plan Available result, and reason/scanned/skipped values. The coordinator supplies the ordered reports and private ReportSummary.

ScannerStatus and AuditReport remain shared mgc-types contracts. This decision is infallible and introduces no local error type. Direct unit cases live under test/.

## Bản tiếng Việt

status_resolution chỉ sở hữu bảng quyết định verdict. Owner giữ thứ tự Failed trước ToolMissing, nhận diện trường hợp mọi ecosystem chưa hỗ trợ, phân xử Available/Partial, kết quả Available của plan rỗng và các giá trị reason/scanned/skipped. Coordinator truyền danh sách report có thứ tự và ReportSummary nội bộ.

ScannerStatus và AuditReport vẫn là contract dùng chung của mgc-types. Phần phân xử không phát sinh lỗi nên không có error type cục bộ. Test trực tiếp nằm trong test/.
