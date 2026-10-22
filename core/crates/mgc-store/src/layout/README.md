# Store layout owner

<!-- English module guide: purpose, source map, and review boundaries. -->
<!-- Hướng dẫn tiếng Việt về chức năng, source và ranh giới review. -->

## Purpose

This owner maps a caller-provided store root to stable child paths. It performs no filesystem access, I/O, validation, or mutation.

## Source map

- `mod.rs` keeps the existing `mgc_store::layout::Layout` API stable.
- `layout.rs` owns the `Layout` value and all path mappings.
- `test/` owns focused path-contract tests.

## Error boundary

There is no local `error.rs`: every operation is a deterministic `PathBuf::join` and cannot fail. Filesystem creation and validation belong to the callers that perform those operations.

## Verification

Run `cargo test -p mgc-store` after the repository-wide source-refactor phase. The tests added here have not been run in this source-only phase.

## Chức năng

Owner này ánh xạ root do caller cung cấp sang các đường dẫn con ổn định. Nó không truy cập filesystem, không I/O, không xác thực và không sửa dữ liệu.

## Ranh giới lỗi

Không tạo `error.rs` vì mọi thao tác chỉ là `PathBuf::join` xác định và không phát sinh lỗi. Việc tạo hoặc xác thực thư mục thuộc về caller thực hiện I/O.
