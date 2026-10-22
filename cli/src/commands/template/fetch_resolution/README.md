# Fetch Resolution

## Trách nhiệm tiếng Việt

Phân giải metadata/tag/archive và tạo cache cho version template được chọn.

## Responsibility / Trách nhiệm

Resolve registry metadata/tags and materialize the requested layer version.

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

- [fetch_resolution.rs](fetch_resolution.rs)

## Error owner / Owner xử lý lỗi

- [error.rs](error.rs) reuses the existing shared error constructors.
- error.rs dùng lại constructor lỗi chung hiện có và không đổi thông điệp.
