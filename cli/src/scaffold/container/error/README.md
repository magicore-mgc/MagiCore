# Container error owner

Owns typed failures for unsupported container lifecycle profiles, missing native manifests and split Web runtime layouts. The errors preserve the existing English diagnostics and flow through anyhow to the shared CLI renderer.

Quản lý lỗi typed khi thiếu profile lifecycle, manifest native hoặc runtime Web bị tách. Giữ nguyên diagnostic tiếng Anh và truyền qua anyhow tới renderer CLI dùng chung.

These errors are deterministic profile/manifest failures and are not retried. The shared CLI renderer handles diagnostics once.
