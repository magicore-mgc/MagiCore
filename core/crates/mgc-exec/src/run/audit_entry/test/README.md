# Audit entry tests

These tests verify the report-to-record boundary, especially that sensitive argv is redacted before audit persistence. Serialization and filesystem append contracts belong to `crate::audit` tests.

# Test audit entry

Test ở đây kiểm tra biên report-to-record, đặc biệt argv nhạy cảm phải được redact trước khi ghi audit. Contract serialize và append file thuộc test của `crate::audit`.
