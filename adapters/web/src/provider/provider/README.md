# NpmDependencyProvider state, registry construction including scoped chains, and per-instance age-policy arming. Changing the age policy clears cached version and optional-enqueue decisions and updates the registry fetch mode. Registry transport and HTTP behavior stay in `native/npm_registry`. / Quản lý state `NpmDependencyProvider`, khởi tạo registry (kể cả chain có scope), và bật age policy theo từng instance. Đổi age policy sẽ xóa cache version/optional-enqueue và cập nhật fetch mode của registry. Transport/HTTP vẫn thuộc `native/npm_registry`.

## Responsibility

NpmDependencyProvider state, registry construction including scoped chains, and per-instance age-policy arming. Changing the age policy clears cached version and optional-enqueue decisions and updates the registry fetch mode. Registry transport and HTTP behavior stay in `native/npm_registry`. Cache invalidation follows each provider instance; scoped registry-construction errors propagate unchanged.

## Scope and boundary

This owner contains only this behavior. `mod.rs` wires the same-name implementation and its nearest tests. It uses shared resolver/package types and keeps the established provider API and errors.

## Phạm vi và ranh giới

Quản lý state `NpmDependencyProvider`, khởi tạo registry (kể cả chain có scope), và bật age policy theo từng instance. Đổi age policy sẽ xóa cache version/optional-enqueue và cập nhật fetch mode của registry. Transport/HTTP vẫn thuộc `native/npm_registry`. Invalidation cache theo từng provider; lỗi khởi tạo registry scoped được truyền nguyên trạng.

Owner này chỉ chứa hành vi trên. `mod.rs` nối implementation cùng tên với test gần nhất. Owner dùng type resolver/package chung và giữ nguyên API provider cùng error contract.
