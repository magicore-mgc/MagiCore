# Audit manifest discovery / Discovery manifest audit

Owns the PEP 751 `pylock.*.toml` filename predicate used by the Python + Rust dispatch gate. It intentionally mirrors the shared scanner and preserves the existing behavior of returning `false` when reading the directory fails.

Quản lý predicate tên file PEP 751 `pylock.*.toml` dùng trong gate dispatch Python + Rust. Owner cố ý khớp scanner dùng chung và giữ cách trả `false` khi đọc directory thất bại.

No manifest files are written and no audit scanner is run here. Add direct filename/error coverage in the later test phase; no test was run in this structural slice.

Owner không ghi manifest và không chạy scanner audit. Bổ sung test trực tiếp tên file/lỗi ở phase test sau; slice cấu trúc này chưa chạy test.
