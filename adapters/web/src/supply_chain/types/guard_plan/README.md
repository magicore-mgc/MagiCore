# GuardPlan

## Responsibility / Trách nhiệm
Combines already-read policy inputs into the exact set of guards that the coordinator runs and the quarantine age passed to the checker.

Kết hợp input policy đã đọc thành tập guard coordinator sẽ chạy và tuổi cách ly truyền cho checker.

## Contract / Hợp đồng
Preserves the existing bypass: MAGICORE_ALLOW_UNTRUSTED suppresses both age and downgrade checks and requests the one-time warning. No input is read here.

Giữ bypass hiện tại: MAGICORE_ALLOW_UNTRUSTED tắt cả guard tuổi và downgrade, đồng thời yêu cầu cảnh báo một lần. Owner này không tự đọc input.
