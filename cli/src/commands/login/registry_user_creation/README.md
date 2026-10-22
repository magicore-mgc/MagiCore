# Registry user creation

EN: Owns the CouchDB-compatible user-registration request and keeps its URL, HTTP exchange, response bounds and token decoding in focused children. The public crate-local `create_registry_user` function remains available through the login facade.

VI: Owner này sở hữu request đăng ký user tương thích CouchDB và tách URL, HTTP, giới hạn response, giải mã token thành các owner con. Function crate-local `create_registry_user` vẫn được export qua facade login.

Security contract preserved from the current source: the shared registry URL policy rejects remote plain HTTP and credential-bearing URLs, loopback HTTP remains allowed, redirects are disabled, timeout stays 15 seconds, response size stays at most 32 KiB, and token text cannot contain whitespace/control bytes. Error messages remain English and response bodies are not included in diagnostics.

Contract bảo mật được giữ theo source hiện tại: shared registry URL policy chặn HTTP từ xa và URL chứa credential, cho phép loopback HTTP, tắt redirect, timeout 15 giây, response tối đa 32 KiB, token không chứa whitespace/control. Diagnostic vẫn tiếng Anh và không đưa response body vào lỗi.

The exact timeout/limit values and observable error strings must be reviewed before any future behavior change. This refactor does not validate real registry compatibility; related tests remain `NOT_RUN`.
