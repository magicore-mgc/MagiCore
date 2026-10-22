# Compatibility settings owner

This owner stores the explicit `[compat]` opt-ins for Git dependency resolution and its host allowlist. It does not resolve dependencies or authorize a host; consumers enforce the configured policy. `CompatConfig` remains available under the original project module path.

Serde defaults remain `allow_git_deps = false` and an empty host list. These DTO fields are not fallible, so no local `error.rs` is added.

# Owner cấu hình tương thích

Owner này lưu opt-in `[compat]` cho phân giải dependency Git và allowlist host. Nó không tự phân giải dependency hay cấp quyền cho host; consumer phải áp dụng policy. `CompatConfig` vẫn được truy cập qua public path project hiện tại.

Serde default vẫn là `allow_git_deps = false` và danh sách host rỗng. DTO không có nhánh lỗi, vì vậy không thêm `error.rs`.
