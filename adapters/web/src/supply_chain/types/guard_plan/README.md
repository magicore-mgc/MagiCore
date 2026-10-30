# GuardPlan

## Responsibility / Trách nhiệm
Combines already-read policy inputs into the exact set of guards that the coordinator runs and the quarantine age passed to the checker.

Kết hợp input policy đã đọc thành tập guard coordinator sẽ chạy và tuổi cách ly truyền cho checker.

## Contract / Hợp đồng
Preserves the existing bypass: MAGICORE_ALLOW_UNTRUSTED suppresses both age and downgrade checks and requests the warning. No input is read here.

The age guard runs when the loaded policy carries an age; the coordinator applies `DEFAULT_MIN_RELEASE_AGE_HOURS` for a first resolution, so a default project with no lockfile is gated by the 24h quarantine without opt-in.

Giữ bypass hiện tại: MAGICORE_ALLOW_UNTRUSTED tắt cả guard tuổi và downgrade, đồng thời yêu cầu cảnh báo. Owner này không tự đọc input.

Guard tuổi chạy khi policy đã đọc có tuổi; coordinator áp `DEFAULT_MIN_RELEASE_AGE_HOURS` cho resolve lần đầu nên project mặc định chưa có lockfile vẫn bị quarantine 24h mà không cần opt-in.
