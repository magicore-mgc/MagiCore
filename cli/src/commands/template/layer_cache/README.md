# Layer Cache

## Trách nhiệm tiếng Việt

Điều phối cache hit/miss, fetch từ registry và fallback chỉ với lỗi availability được cho phép.

## Responsibility / Trách nhiệm

Resolve cache hit/miss, registry fallback and availability-only fallback classification.

## Boundaries / Ranh giới

**English:** Errors reuse the existing shared constructors, types, messages and propagation paths.

**Tiếng Việt:** Lỗi dùng lại constructor, kiểu, thông điệp và cách truyền lỗi hiện có; không tạo error placeholder.

## Ownership / Sở hữu

- English: mod.rs exports this owner surface to its parent.
- Tiếng Việt: mod.rs export API của owner lên module cha.
- English: the same-name Rust file contains implementation.
- Tiếng Việt: file Rust cùng tên chứa phần triển khai.
- English: this folder adds no network, cache, fallback or CLI behavior.
- Tiếng Việt: thư mục không thêm hành vi mạng, cache, fallback hay CLI mới.
## Implementation / Cài đặt

- `entry/`, `validation/`, `flow/`, `ref_match/`, `error_mapping/` owners (the former `layer_cache.rs` body)

## Error owner / Owner xử lý lỗi

- [error.rs](error.rs) reuses the existing shared error constructors.
- error.rs dùng lại constructor lỗi chung hiện có và không đổi thông điệp.
