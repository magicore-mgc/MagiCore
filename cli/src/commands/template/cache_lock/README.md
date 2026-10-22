# Cache Lock

## Trách nhiệm tiếng Việt

Tạo và khóa file đồng bộ layer giữa các tiến trình; giữ nguyên tên/key lock cũ.

## Responsibility / Trách nhiệm

Cross-process layer lock directory/file safety and stable lock-key naming.

## Boundaries / Ranh giới

**English:** error.rs owns the two existing lock-validation messages; I/O errors still propagate unchanged.

**Tiếng Việt:** error.rs sở hữu hai thông điệp kiểm tra lock hiện có; lỗi I/O vẫn được truyền nguyên trạng.

## Ownership / Sở hữu

- English: mod.rs exports this owner surface to its parent.
- Tiếng Việt: mod.rs export API của owner lên module cha.
- English: the same-name Rust file contains implementation.
- Tiếng Việt: file Rust cùng tên chứa phần triển khai.
- English: this folder adds no network, cache, fallback or CLI behavior.
- Tiếng Việt: thư mục không thêm hành vi mạng, cache, fallback hay CLI mới.
## Implementation / Cài đặt

- [cache_lock.rs](cache_lock.rs)

- [Local lock errors](error.rs)

## Error owner / Owner xử lý lỗi

- [error.rs](error.rs) reuses the existing shared error constructors.
- error.rs dùng lại constructor lỗi chung hiện có và không đổi thông điệp.
