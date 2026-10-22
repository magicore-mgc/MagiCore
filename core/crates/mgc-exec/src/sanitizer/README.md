# Sanitizer

Owns argv redaction before audit serialization. The public coordinator preserves argument order and only transforms recorded audit data; execution receives its original argv elsewhere.

## Contract

Sensitive `flag=value` values, the next argv value after a sensitive flag, and a whole shell-like argv item containing a sensitive flag are redacted. The stable public entry point is `redact_args`.

# Sanitizer — tiếng Việt

Sở hữu việc che argv trước khi serialize audit. Coordinator công khai giữ nguyên thứ tự đối số và chỉ đổi dữ liệu được ghi vào audit; argv gửi thực thi được xử lý riêng.

## Hợp đồng

Che value của `flag=value`, argv kế tiếp sau flag nhạy cảm và toàn bộ argv dạng chuỗi shell có chứa flag nhạy cảm. Entry point công khai ổn định là `redact_args`.
