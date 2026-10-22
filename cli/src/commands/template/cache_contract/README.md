# Cache Contract

## Trách nhiệm tiếng Việt

Kiểm tra contract manifest và đọc version marker của cache.

## Responsibility / Trách nhiệm

Validate template manifest contract and read cached version metadata.

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

- [cache_contract.rs](cache_contract.rs)

## Error owner / Owner xử lý lỗi

- [error.rs](error.rs) reuses the existing shared error constructors.
- error.rs dùng lại constructor lỗi chung hiện có và không đổi thông điệp.
