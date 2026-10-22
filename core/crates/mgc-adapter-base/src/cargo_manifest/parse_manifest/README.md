# Cargo manifest parser

Owns TOML reading, supported dependency-shape validation, and conversion into the shared `Manifest`. Unsupported Cargo semantics are rejected with the existing capability error instead of being silently dropped.

Sở hữu đọc TOML, kiểm tra cấu trúc dependency hỗ trợ và chuyển sang `Manifest` dùng chung. Semantics Cargo chưa hỗ trợ bị từ chối bằng capability error hiện hành, không bị bỏ qua im lặng.
