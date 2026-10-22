# Publish authentication

EN: This folder preserves the crate's public auth API and wires the type, header formatting, redacted Debug implementation, and credential resolver owners.
VI: Thư mục này giữ API auth công khai của crate và nối các owner cho kiểu dữ liệu, header, Debug đã che bí mật và bộ phân giải credentials.

Boundary: Keep credential precedence, public paths, diagnostics, and redaction behavior unchanged in this structural extraction.
Test scope: Each behavior owner registers tests in its adjacent test/ folder; those tests are authored but not run in the source-first phase.
