# Publish-age quarantine

## Responsibility / Trách nhiệm
Checks resolved package versions against registry publish timestamps using the configured minimum age.

Kiểm tra version package đã resolve theo timestamp phát hành registry và minimum age cấu hình.

## Contract / Hợp đồng
An active age guard requires package metadata. Missing or unparsable timestamps fail closed unless the project explicitly enables `allow_missing_time`; that escape emits a process-local warning. The low-level predicate returns `Ok(false)` for both absent and unparsable timestamps, matching its documented unknown-time result, and the guard handles that value explicitly. Nonpositive thresholds disable the age check; thresholds outside Chrono's supported duration range return a typed error.

Guard tuổi đang bật yêu cầu metadata package. Timestamp thiếu hoặc không parse được bị chặn, trừ khi project bật tường minh `allow_missing_time`; escape đó phát cảnh báo một lần trong process. Predicate cấp thấp trả `Ok(false)` cho timestamp thiếu/sai như kết quả unknown đã mô tả, còn guard xử lý rõ giá trị đó. Ngưỡng không dương vẫn tắt guard; ngưỡng ngoài phạm vi Chrono trả lỗi typed.
