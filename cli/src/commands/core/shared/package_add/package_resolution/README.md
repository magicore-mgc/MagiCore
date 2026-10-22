# Add package resolution

Processes requested packages serially through the existing adapter, updates the in-memory manifest delta, and records packages that actually changed. The caller retains the mutation lock across this awaited phase.

## Tiếng Việt

Resolve package theo thứ tự qua adapter hiện tại, cập nhật manifest delta trong bộ nhớ và ghi nhận package thực sự thay đổi. Caller tiếp tục giữ mutation lock trong toàn bộ phase await này.
