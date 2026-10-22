# Store Paths

## Responsibility

Resolves project-local store, lock and virtual-store paths.

## Trách nhiệm

Phân giải path store, lock, virtual-store của project.

The Web cache root is `.magicore/cache/web`; Web install uses that path both as `Layout::root()` and as the project-lock key. Lock files live under its `locks/` child. Do not substitute the project root or user-global MGC store when acquiring that lock.

Cache root Web là `.magicore/cache/web`; Web install dùng path đó làm `Layout::root()` và project-lock key. File lock nằm dưới `locks/`. Không thay bằng project root hoặc global MGC store khi lấy khóa.

Implementation: `store_paths.rs`; `mod.rs` only wires this owner.

Triển khai: `store_paths.rs`; `mod.rs` chỉ nối owner này.
